use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
 
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&url).await?;
    println!("Connected to the database!");
 
    Ok(())
}