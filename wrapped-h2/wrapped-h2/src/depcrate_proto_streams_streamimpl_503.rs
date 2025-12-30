// Generated macro for impl_503 (impl)
macro_rules! Depcrate_proto_streams_streamimpl_503 {
() => {
// Module: crate::proto::streams::stream
// Provides: {"impl_503"}
// Dependencies: {}
impl store :: Next for NextSendCapacity { fn next (stream : & Stream) -> Option < store :: Key > { stream . next_pending_send_capacity } fn set_next (stream : & mut Stream , key : Option < store :: Key >) { stream . next_pending_send_capacity = key ; } fn take_next (stream : & mut Stream) -> Option < store :: Key > { stream . next_pending_send_capacity . take () } fn is_queued (stream : & Stream) -> bool { stream . is_pending_send_capacity } fn set_queued (stream : & mut Stream , val : bool) { stream . is_pending_send_capacity = val ; } }
};
}
