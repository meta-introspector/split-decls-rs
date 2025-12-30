// Generated macro for impl_505 (impl)
macro_rules! Depcrate_proto_streams_streamimpl_505 {
() => {
// Module: crate::proto::streams::stream
// Provides: {"impl_505"}
// Dependencies: {}
impl store :: Next for NextOpen { fn next (stream : & Stream) -> Option < store :: Key > { stream . next_open } fn set_next (stream : & mut Stream , key : Option < store :: Key >) { stream . next_open = key ; } fn take_next (stream : & mut Stream) -> Option < store :: Key > { stream . next_open . take () } fn is_queued (stream : & Stream) -> bool { stream . is_pending_open } fn set_queued (stream : & mut Stream , val : bool) { if val { debug_assert ! (! stream . is_pending_send) ; } stream . is_pending_open = val ; } }
};
}
