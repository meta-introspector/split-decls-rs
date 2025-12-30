// Generated macro for CompressError (enum)
macro_rules! Depcrate_zlib_stream_deflateCompressError {
() => {
// Module: crate::zlib::stream::deflate
// Provides: {"CompressError"}
// Dependencies: {}
# [doc = " The error produced by [`Compress::compress()`]."] # [derive (Debug , thiserror :: Error)] # [error ("{msg}")] # [allow (missing_docs)] pub enum CompressError { # [error ("stream error")] StreamError , # [error ("Not enough memory")] InsufficientMemory , # [error ("An unknown error occurred: {err}")] Unknown { err : c_int } , }
};
}
