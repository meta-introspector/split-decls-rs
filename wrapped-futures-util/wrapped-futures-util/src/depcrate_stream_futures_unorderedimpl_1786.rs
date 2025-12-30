// Generated macro for impl_1786 (impl)
macro_rules! Depcrate_stream_futures_unorderedimpl_1786 {
() => {
// Module: crate::stream::futures_unordered
// Provides: {"impl_1786"}
// Dependencies: {}
impl < Fut : Future > FusedStream for FuturesUnordered < Fut > { fn is_terminated (& self) -> bool { self . is_terminated . load (Relaxed) } }
};
}
