// Generated macro for quiche_stream_iter_free (function)
macro_rules! Depcrate_ffiquiche_stream_iter_free {
() => {
// Module: crate::ffi
// Provides: {"quiche_stream_iter_free"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_stream_iter_free (iter : * mut StreamIter) { drop (unsafe { Box :: from_raw (iter) }) ; }
};
}
