// Generated macro for stream_to_string (function)
macro_rules! Depcrate_to_stringstream_to_string {
() => {
// Module: crate::to_string
// Provides: {"stream_to_string"}
// Dependencies: {}
# [doc = "\nFormat a value into a string.\n\nThis method will use a default format that's like Rust's `Debug`.\n"] pub fn stream_to_string (v : impl sval :: Value) -> String { let mut out = String :: new () ; crate :: stream_to_write (& mut out , v) . expect ("infallible write") ; out }
};
}
