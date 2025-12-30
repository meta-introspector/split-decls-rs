// Generated macro for impl_276 (impl)
macro_rules! Depcrate_future_pendingimpl_276 {
() => {
// Module: crate::future::pending
// Provides: {"impl_276"}
// Dependencies: {}
impl < T > Future for Pending < T > { type Output = T ; fn poll (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < T > { Poll :: Pending } }
};
}
