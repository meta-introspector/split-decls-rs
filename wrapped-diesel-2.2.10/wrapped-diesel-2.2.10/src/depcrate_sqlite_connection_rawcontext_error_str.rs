// Generated macro for context_error_str (function)
macro_rules! Depcrate_sqlite_connection_rawcontext_error_str {
() => {
// Module: crate::sqlite::connection::raw
// Provides: {"context_error_str"}
// Dependencies: {}
unsafe fn context_error_str (ctx : * mut ffi :: sqlite3_context , error : & str) { let len : i32 = error . len () . try_into () . expect ("Trying to set a error message with more than 2^32 byte is not supported") ; ffi :: sqlite3_result_error (ctx , error . as_ptr () as * const _ , len) ; }
};
}
