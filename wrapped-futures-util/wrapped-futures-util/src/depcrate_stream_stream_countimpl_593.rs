// Generated macro for impl_593 (impl)
macro_rules! Depcrate_stream_stream_countimpl_593 {
() => {
// Module: crate::stream::stream::count
// Provides: {"impl_593"}
// Dependencies: {}
impl < St : FusedStream > FusedFuture for Count < St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
};
}
