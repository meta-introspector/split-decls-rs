// Generated macro for impl_152 (impl)
macro_rules! Depcrate_interleave_pendingimpl_152 {
() => {
// Module: crate::interleave_pending
// Provides: {"impl_152"}
// Dependencies: {}
impl < Fut : Future > Future for InterleavePending < Fut > { type Output = Fut :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . poll_with (cx , Fut :: poll) } }
};
}
