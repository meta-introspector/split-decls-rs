// Generated macro for stream_to_write (function)
macro_rules! Depcrate_to_writestream_to_write {
() => {
// Module: crate::to_write
// Provides: {"stream_to_write"}
// Dependencies: {}
# [doc = "\nFormat a value into an underlying formatter.\n\nThis method will use a default format that's like Rust's `Debug`.\n"] pub fn stream_to_write (fmt : impl Write , v : impl sval :: Value) -> fmt :: Result { v . stream (& mut Writer :: new (GenericWriter (fmt))) . map_err (| _ | fmt :: Error) }
};
}
