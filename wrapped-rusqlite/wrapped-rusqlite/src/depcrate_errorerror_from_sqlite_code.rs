// Generated macro for error_from_sqlite_code (function)
macro_rules! Depcrate_errorerror_from_sqlite_code {
() => {
// Module: crate::error
// Provides: {"error_from_sqlite_code"}
// Dependencies: {}
# [cold] pub fn error_from_sqlite_code (code : c_int , message : Option < String >) -> Error { Error :: SqliteFailure (ffi :: Error :: new (code) , message) }
};
}
