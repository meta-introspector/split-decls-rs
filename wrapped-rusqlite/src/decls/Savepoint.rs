macro_rules! deps {
    () => {
        DropBehavior!();
        Connection!();
        Result!();
    };
}

macro_rules! Savepoint {
    () => {
        deps!();
        # [doc = " Represents a savepoint on a database connection."] # [doc = ""] # [doc = " ## Note"] # [doc = ""] # [doc = " Savepoints will roll back by default. Use `commit` method to explicitly"] # [doc = " commit the savepoint, or use `set_drop_behavior` to change what happens"] # [doc = " when the savepoint is dropped."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use rusqlite::{Connection, Result};"] # [doc = " # fn do_queries_part_1(_conn: &Connection) -> Result<()> { Ok(()) }"] # [doc = " # fn do_queries_part_2(_conn: &Connection) -> Result<()> { Ok(()) }"] # [doc = " fn perform_queries(conn: &mut Connection) -> Result<()> {"] # [doc = "     let sp = conn.savepoint()?;"] # [doc = ""] # [doc = "     do_queries_part_1(&sp)?; // sp causes rollback if this fails"] # [doc = "     do_queries_part_2(&sp)?; // sp causes rollback if this fails"] # [doc = ""] # [doc = "     sp.commit()"] # [doc = " }"] # [doc = " ```"] # [derive (Debug)] pub struct Savepoint < 'conn > { conn : & 'conn Connection , name : String , drop_behavior : DropBehavior , committed : bool , }
    };
}

Savepoint!()