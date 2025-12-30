// Generated macro for impl_3824 (impl)
macro_rules! Depcrate_sqlite_connection_stmtimpl_3824 {
() => {
// Module: crate::sqlite::connection::stmt
// Provides: {"impl_3824"}
// Dependencies: {}
impl Drop for Statement { fn drop (& mut self) { use std :: thread :: panicking ; let raw_connection = self . raw_connection () ; let finalize_result = unsafe { ffi :: sqlite3_finalize (self . inner_statement . as_ptr ()) } ; if let Err (e) = ensure_sqlite_ok (finalize_result , raw_connection) { if panicking () { write ! (stderr () , "Error finalizing SQLite prepared statement: {e:?}") . expect ("Error writing to `stderr`") ; } else { panic ! ("Error finalizing SQLite prepared statement: {:?}" , e) ; } } } }
};
}
