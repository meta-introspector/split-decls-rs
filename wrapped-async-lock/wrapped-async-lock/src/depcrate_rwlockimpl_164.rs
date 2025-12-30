// Generated macro for impl_164 (impl)
macro_rules! Depcrate_rwlockimpl_164 {
() => {
// Module: crate::rwlock
// Provides: {"impl_164"}
// Dependencies: {}
impl < T : ? Sized > Drop for RwLockWriteGuardArc < T > { # [inline] fn drop (& mut self) { unsafe { self . lock . raw . write_unlock () ; } } }
};
}
