// Generated macro for stream_to_string (function)
macro_rules! Depcrate_to_stringstream_to_string {
() => {
// Module: crate::to_string
// Provides: {"stream_to_string"}
// Dependencies: {}
# [doc = "\nStream a value as JSON into a string.\n\nThis method will fail if the value contains complex values as keys.\n"] pub fn stream_to_string (v : impl sval :: Value) -> Result < String , Error > { let mut out = String :: new () ; crate :: stream_to_fmt_write (& mut out , v) ? ; Ok (out) }
};
}
