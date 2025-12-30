// Generated macro for Transaction (struct)
macro_rules! Depcrate_transactionTransaction {
() => {
// Module: crate::transaction
// Provides: {"Transaction"}
// Dependencies: {}
# [doc = " Represents a transaction on a database connection."] # [doc = ""] # [doc = " ## Note"] # [doc = ""] # [doc = " Transactions will roll back by default. Use `commit` method to explicitly"] # [doc = " commit the transaction, or use `set_drop_behavior` to change what happens"] # [doc = " when the transaction is dropped."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use rusqlite::{Connection, Result};"] # [doc = " # fn do_queries_part_1(_conn: &Connection) -> Result<()> { Ok(()) }"] # [doc = " # fn do_queries_part_2(_conn: &Connection) -> Result<()> { Ok(()) }"] # [doc = " fn perform_queries(conn: &mut Connection) -> Result<()> {"] # [doc = "     let tx = conn.transaction()?;"] # [doc = ""] # [doc = "     do_queries_part_1(&tx)?; // tx causes rollback if this fails"] # [doc = "     do_queries_part_2(&tx)?; // tx causes rollback if this fails"] # [doc = ""] # [doc = "     tx.commit()"] # [doc = " }"] # [doc = " ```"] # [derive (Debug)] pub struct Transaction < 'conn > { conn : & 'conn Connection , drop_behavior : DropBehavior , }
};
}
