// Generated macro for impl_131 (impl)
macro_rules! Depcrate_rwlockimpl_131 {
() => {
// Module: crate::rwlock
// Provides: {"impl_131"}
// Dependencies: {}
impl < T > Drop for RwLockReadGuardArc < T > { # [inline] fn drop (& mut self) { unsafe { let arc = ManuallyDrop :: into_inner (Self :: inner_arc (self)) ; arc . raw . read_unlock () ; } } }
};
}
