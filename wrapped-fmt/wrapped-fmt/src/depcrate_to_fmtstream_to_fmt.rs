// Generated macro for stream_to_fmt (function)
macro_rules! Depcrate_to_fmtstream_to_fmt {
() => {
// Module: crate::to_fmt
// Provides: {"stream_to_fmt"}
// Dependencies: {}
# [doc = "\nFormat a value into an underlying formatter.\n"] pub fn stream_to_fmt (fmt : & mut fmt :: Formatter , v : impl sval :: Value) -> fmt :: Result { v . stream (& mut Writer :: new (fmt)) . map_err (| _ | fmt :: Error) }
};
}
