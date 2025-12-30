// Generated macro for impl_2606 (impl)
macro_rules! Depcrate_lock_bilockimpl_2606 {
() => {
// Module: crate::lock::bilock
// Provides: {"impl_2606"}
// Dependencies: {}
# [cfg (feature = "bilock")] impl < 'a , T > Future for BiLockAcquire < 'a , T > { type Output = BiLockGuard < 'a , T > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . bilock . poll_lock (cx) } }
};
}
