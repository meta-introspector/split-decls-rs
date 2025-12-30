// Generated macro for IterStream (struct)
macro_rules! Depcrate_stream_iterIterStream {
() => {
// Module: crate::stream::iter
// Provides: {"IterStream"}
// Dependencies: {}
# [doc = " A stream which is just a shim over an underlying instance of `Iterator`."] # [doc = ""] # [doc = " This stream will never block and is always ready."] pub struct IterStream < I > { iter : I , }
};
}
