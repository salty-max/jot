use rusqlite::Connection;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("notes.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id integer primary key,
            body text not null unique
        )",
        [],
        )?;
    Ok(())
}
