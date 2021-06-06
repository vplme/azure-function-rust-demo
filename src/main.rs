use serde::{Deserialize};
use actix_web::{get, App, HttpResponse, HttpServer, HttpRequest, Result, web};
use std::env;
use std::net::Ipv4Addr;
use actix_web::web::Query;


#[derive(Deserialize)]
struct HelloQueryInfo {
    name: Option<String>,
}


#[get("/HelloWorld")]
async fn hello(info: Query<HelloQueryInfo>) -> Result<HttpResponse> {
    let name = info.name.as_deref().unwrap_or("World");
    let text = format!("Hello {}!", name);
    Ok(HttpResponse::Ok().body(text))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
            .service(
                web::scope("/api")
                    .service(hello)
            )
        )
        .bind((Ipv4Addr::UNSPECIFIED, get_port()))?
        .run()
        .await
}

fn get_port() -> u16 {
    const PORT_KEY: &str = "FUNCTIONS_CUSTOMHANDLER_PORT";
    match env::var(PORT_KEY) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 4000,
    }
}
