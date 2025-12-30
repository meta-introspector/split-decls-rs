// Generated macro for impl_2642 (impl)
macro_rules! Depcrate_lock_muteximpl_2642 {
() => {
// Module: crate::lock::mutex
// Provides: {"impl_2642"}
// Dependencies: {}
impl < T : ? Sized > FusedFuture for MutexLockFuture < '_ , T > { fn is_terminated (& self) -> bool { self . mutex . is_none () } }
};
}
