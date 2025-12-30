// Generated macro for to_sqlite_error (function)
macro_rules! Depcrate_errorto_sqlite_error {
() => {
// Module: crate::error
// Provides: {"to_sqlite_error"}
// Dependencies: {}
# [doc = " Transform Rust error to SQLite error (message and code)."] # [doc = " # Safety"] # [doc = " This function is unsafe because it uses raw pointer"] pub unsafe fn to_sqlite_error (e : & Error , err_msg : * mut * mut c_char) -> c_int { use crate :: util :: alloc ; match e { Error :: SqliteFailure (err , s) => { if let Some (s) = s { * err_msg = alloc (s) ; } err . extended_code } err => { * err_msg = alloc (& err . to_string ()) ; ffi :: SQLITE_ERROR } } }
};
}
