// Generated macro for quiche_stream_iter_next (function)
macro_rules! Depcrate_ffiquiche_stream_iter_next {
() => {
// Module: crate::ffi
// Provides: {"quiche_stream_iter_next"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_stream_iter_next (iter : & mut StreamIter , stream_id : * mut u64 ,) -> bool { if let Some (v) = iter . next () { unsafe { * stream_id = v } ; return true ; } false }
};
}
