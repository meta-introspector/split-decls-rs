// Generated macro for last_error_message (function)
macro_rules! Depcrate_sqlite_connection_stmtlast_error_message {
() => {
// Module: crate::sqlite::connection::stmt
// Provides: {"last_error_message"}
// Dependencies: {}
fn last_error_message (conn : * mut ffi :: sqlite3) -> String { let c_str = unsafe { CStr :: from_ptr (ffi :: sqlite3_errmsg (conn)) } ; c_str . to_string_lossy () . into_owned () }
};
}
