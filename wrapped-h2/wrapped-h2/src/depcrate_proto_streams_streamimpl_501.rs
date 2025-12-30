// Generated macro for impl_501 (impl)
macro_rules! Depcrate_proto_streams_streamimpl_501 {
() => {
// Module: crate::proto::streams::stream
// Provides: {"impl_501"}
// Dependencies: {}
impl store :: Next for NextAccept { fn next (stream : & Stream) -> Option < store :: Key > { stream . next_pending_accept } fn set_next (stream : & mut Stream , key : Option < store :: Key >) { stream . next_pending_accept = key ; } fn take_next (stream : & mut Stream) -> Option < store :: Key > { stream . next_pending_accept . take () } fn is_queued (stream : & Stream) -> bool { stream . is_pending_accept } fn set_queued (stream : & mut Stream , val : bool) { stream . is_pending_accept = val ; } }
};
}
