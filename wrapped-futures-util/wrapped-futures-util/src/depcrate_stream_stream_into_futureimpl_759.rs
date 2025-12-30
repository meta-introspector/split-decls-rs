// Generated macro for impl_759 (impl)
macro_rules! Depcrate_stream_stream_into_futureimpl_759 {
() => {
// Module: crate::stream::stream::into_future
// Provides: {"impl_759"}
// Dependencies: {}
impl < St : Stream + Unpin > FusedFuture for StreamFuture < St > { fn is_terminated (& self) -> bool { self . stream . is_none () } }
};
}
