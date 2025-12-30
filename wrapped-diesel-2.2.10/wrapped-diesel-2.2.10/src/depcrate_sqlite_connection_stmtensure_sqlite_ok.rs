// Generated macro for ensure_sqlite_ok (function)
macro_rules! Depcrate_sqlite_connection_stmtensure_sqlite_ok {
() => {
// Module: crate::sqlite::connection::stmt
// Provides: {"ensure_sqlite_ok"}
// Dependencies: {}
pub (super) fn ensure_sqlite_ok (code : libc :: c_int , raw_connection : * mut ffi :: sqlite3 ,) -> QueryResult < () > { if code == ffi :: SQLITE_OK { Ok (()) } else { Err (last_error (raw_connection)) } }
};
}
