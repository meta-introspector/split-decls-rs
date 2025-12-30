// Generated macro for impl_153 (impl)
macro_rules! Depcrate_interleave_pendingimpl_153 {
() => {
// Module: crate::interleave_pending
// Provides: {"impl_153"}
// Dependencies: {}
impl < Fut : FusedFuture > FusedFuture for InterleavePending < Fut > { fn is_terminated (& self) -> bool { self . inner . is_terminated () } }
};
}
