// Generated macro for last_error_code (function)
macro_rules! Depcrate_sqlite_connection_stmtlast_error_code {
() => {
// Module: crate::sqlite::connection::stmt
// Provides: {"last_error_code"}
// Dependencies: {}
fn last_error_code (conn : * mut ffi :: sqlite3) -> libc :: c_int { unsafe { ffi :: sqlite3_extended_errcode (conn) } }
};
}
