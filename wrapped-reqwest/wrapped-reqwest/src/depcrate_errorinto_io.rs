// Generated macro for into_io (function)
macro_rules! Depcrate_errorinto_io {
() => {
// Module: crate::error
// Provides: {"into_io"}
// Dependencies: {}
# [cfg (any (feature = "gzip" , feature = "zstd" , feature = "brotli" , feature = "deflate" , feature = "blocking" ,))] pub (crate) fn into_io (e : BoxError) -> io :: Error { io :: Error :: new (io :: ErrorKind :: Other , e) }
};
}
