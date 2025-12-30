// Generated macro for impl_789 (impl)
macro_rules! Depcrate_stream_stream_nextimpl_789 {
() => {
// Module: crate::stream::stream::next
// Provides: {"impl_789"}
// Dependencies: {}
impl < St : ? Sized + FusedStream + Unpin > FusedFuture for Next < '_ , St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
};
}
