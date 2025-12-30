// Generated macro for error_message (function)
macro_rules! Depcrate_sqlite_connectionerror_message {
() => {
// Module: crate::sqlite::connection
// Provides: {"error_message"}
// Dependencies: {}
fn error_message (err_code : libc :: c_int) -> & 'static str { ffi :: code_to_str (err_code) }
};
}
