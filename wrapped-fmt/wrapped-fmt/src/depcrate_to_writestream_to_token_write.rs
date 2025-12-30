// Generated macro for stream_to_token_write (function)
macro_rules! Depcrate_to_writestream_to_token_write {
() => {
// Module: crate::to_write
// Provides: {"stream_to_token_write"}
// Dependencies: {}
# [doc = "\nFormat a value into an underlying token-aware formatter.\n\nThis method is like [`stream_to_write`], but can be used to customize the way\nvalues are formatted through the implementation of [`TokenWrite`].\n"] pub fn stream_to_token_write (fmt : impl TokenWrite , v : impl sval :: Value) -> fmt :: Result { v . stream (& mut Writer :: new (fmt)) . map_err (| _ | fmt :: Error) }
};
}
