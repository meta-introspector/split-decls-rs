// Generated macro for impl_506 (impl)
macro_rules! Depcrate_proto_streams_streamimpl_506 {
() => {
// Module: crate::proto::streams::stream
// Provides: {"impl_506"}
// Dependencies: {}
impl store :: Next for NextResetExpire { fn next (stream : & Stream) -> Option < store :: Key > { stream . next_reset_expire } fn set_next (stream : & mut Stream , key : Option < store :: Key >) { stream . next_reset_expire = key ; } fn take_next (stream : & mut Stream) -> Option < store :: Key > { stream . next_reset_expire . take () } fn is_queued (stream : & Stream) -> bool { stream . reset_at . is_some () } fn set_queued (stream : & mut Stream , val : bool) { if val { stream . reset_at = Some (Instant :: now ()) ; } else { stream . reset_at = None ; } } }
};
}
