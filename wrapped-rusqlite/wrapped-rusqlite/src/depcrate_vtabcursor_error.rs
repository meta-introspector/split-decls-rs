// Generated macro for cursor_error (function)
macro_rules! Depcrate_vtabcursor_error {
() => {
// Module: crate::vtab
// Provides: {"cursor_error"}
// Dependencies: {}
# [doc = " Virtual table cursors can set an error message by assigning a string to"] # [doc = " `zErrMsg`."] # [cold] unsafe fn cursor_error < T > (cursor : * mut sqlite3_vtab_cursor , result : Result < T >) -> c_int { match result { Ok (_) => ffi :: SQLITE_OK , Err (Error :: SqliteFailure (err , s)) => { if let Some (err_msg) = s { set_err_msg ((* cursor) . pVtab , & err_msg) ; } err . extended_code } Err (err) => { set_err_msg ((* cursor) . pVtab , & err . to_string ()) ; ffi :: SQLITE_ERROR } } }
};
}
