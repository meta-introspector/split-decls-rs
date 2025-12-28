macro_rules! deps {
    () => {
        Connection!();
        Result!();
    };
}

macro_rules! Null {
    () => {
        deps!();
        # [doc = " Empty struct that can be used to fill in a query parameter as `NULL`."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use rusqlite::{Connection, Result};"] # [doc = " # use rusqlite::types::{Null};"] # [doc = ""] # [doc = " fn insert_null(conn: &Connection) -> Result<usize> {"] # [doc = "     conn.execute(\"INSERT INTO people (name) VALUES (?1)\", [Null])"] # [doc = " }"] # [doc = " ```"] # [derive (Copy , Clone)] pub struct Null ;
    };
}

Null!()