// Generated macro for impl_502 (impl)
macro_rules! Depcrate_proto_streams_streamimpl_502 {
() => {
// Module: crate::proto::streams::stream
// Provides: {"impl_502"}
// Dependencies: {}
impl store :: Next for NextSend { fn next (stream : & Stream) -> Option < store :: Key > { stream . next_pending_send } fn set_next (stream : & mut Stream , key : Option < store :: Key >) { stream . next_pending_send = key ; } fn take_next (stream : & mut Stream) -> Option < store :: Key > { stream . next_pending_send . take () } fn is_queued (stream : & Stream) -> bool { stream . is_pending_send } fn set_queued (stream : & mut Stream , val : bool) { if val { debug_assert ! (! stream . is_pending_open) ; } stream . is_pending_send = val ; } }
};
}
