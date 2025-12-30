// Generated macro for impl_1579 (impl)
macro_rules! Depcrate_stream_onceimpl_1579 {
() => {
// Module: crate::stream::once
// Provides: {"impl_1579"}
// Dependencies: {}
impl < Fut : Future > FusedStream for Once < Fut > { fn is_terminated (& self) -> bool { self . future . is_none () } }
};
}
