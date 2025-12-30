// Generated macro for impl_504 (impl)
macro_rules! Depcrate_proto_streams_streamimpl_504 {
() => {
// Module: crate::proto::streams::stream
// Provides: {"impl_504"}
// Dependencies: {}
impl store :: Next for NextWindowUpdate { fn next (stream : & Stream) -> Option < store :: Key > { stream . next_window_update } fn set_next (stream : & mut Stream , key : Option < store :: Key >) { stream . next_window_update = key ; } fn take_next (stream : & mut Stream) -> Option < store :: Key > { stream . next_window_update . take () } fn is_queued (stream : & Stream) -> bool { stream . is_pending_window_update } fn set_queued (stream : & mut Stream , val : bool) { stream . is_pending_window_update = val ; } }
};
}
