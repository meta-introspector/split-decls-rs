// Generated macro for error_from_handle (function)
macro_rules! Depcrate_errorerror_from_handle {
() => {
// Module: crate::error
// Provides: {"error_from_handle"}
// Dependencies: {}
# [cold] pub unsafe fn error_from_handle (db : * mut ffi :: sqlite3 , code : c_int) -> Error { error_from_sqlite_code (code , error_msg (db , code)) }
};
}
