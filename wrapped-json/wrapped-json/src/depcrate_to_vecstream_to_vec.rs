// Generated macro for stream_to_vec (function)
macro_rules! Depcrate_to_vecstream_to_vec {
() => {
// Module: crate::to_vec
// Provides: {"stream_to_vec"}
// Dependencies: {}
# [doc = "\nStream a value as JSON into a byte buffer.\n\nThis method will fail if the value contains complex values as keys.\n"] pub fn stream_to_vec (v : impl sval :: Value) -> Result < Vec < u8 > , Error > { let mut out = Vec :: new () ; crate :: stream_to_io_write (& mut out , v) ? ; Ok (out) }
};
}
