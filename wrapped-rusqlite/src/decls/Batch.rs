macro_rules! deps {
    () => {
        Connection!();
        Result!();
    };
}

macro_rules! Batch {
    () => {
        deps!();
        # [doc = " Batch fallible iterator"] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " There is no recovery on parsing error, when a invalid statement is found in `sql`, SQLite cannot jump to the next statement."] # [doc = " So you should break the loop when an error is raised by the `next` method."] # [doc = ""] # [doc = " ```rust"] # [doc = " use fallible_iterator::FallibleIterator;"] # [doc = " use rusqlite::{Batch, Connection, Result};"] # [doc = ""] # [doc = " fn main() -> Result<()> {"] # [doc = "     let conn = Connection::open_in_memory()?;"] # [doc = "     let sql = r\""] # [doc = "     CREATE TABLE tbl1 (col);"] # [doc = "     CREATE TABLE tbl2 (col);"] # [doc = "     \";"] # [doc = "     let mut batch = Batch::new(&conn, sql);"] # [doc = "     while let Some(mut stmt) = batch.next()? {"] # [doc = "         stmt.execute([])?;"] # [doc = "     }"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [derive (Debug)] pub struct Batch < 'conn , 'sql > { conn : & 'conn Connection , sql : & 'sql str , tail : usize , }
    };
}

Batch!()