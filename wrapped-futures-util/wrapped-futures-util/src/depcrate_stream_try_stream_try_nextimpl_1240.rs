// Generated macro for impl_1240 (impl)
macro_rules! Depcrate_stream_try_stream_try_nextimpl_1240 {
() => {
// Module: crate::stream::try_stream::try_next
// Provides: {"impl_1240"}
// Dependencies: {}
impl < St : ? Sized + TryStream + Unpin + FusedStream > FusedFuture for TryNext < '_ , St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
};
}
