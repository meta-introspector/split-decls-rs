// Generated macro for error_msg (function)
macro_rules! Depcrate_errorerror_msg {
() => {
// Module: crate::error
// Provides: {"error_msg"}
// Dependencies: {}
unsafe fn error_msg (db : * mut ffi :: sqlite3 , code : c_int) -> Option < String > { if db . is_null () || ffi :: sqlite3_errcode (db) != code { let err_str = ffi :: sqlite3_errstr (code) ; if err_str . is_null () { None } else { Some (errmsg_to_string (err_str)) } } else { Some (errmsg_to_string (ffi :: sqlite3_errmsg (db))) } }
};
}
