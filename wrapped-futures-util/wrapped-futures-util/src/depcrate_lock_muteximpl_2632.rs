// Generated macro for impl_2632 (impl)
macro_rules! Depcrate_lock_muteximpl_2632 {
() => {
// Module: crate::lock::mutex
// Provides: {"impl_2632"}
// Dependencies: {}
impl < T : ? Sized > FusedFuture for OwnedMutexLockFuture < T > { fn is_terminated (& self) -> bool { self . mutex . is_none () } }
};
}
