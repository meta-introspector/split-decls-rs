// Generated macro for impl_123 (impl)
macro_rules! Depcrate_rwlockimpl_123 {
() => {
// Module: crate::rwlock
// Provides: {"impl_123"}
// Dependencies: {}
impl < T : ? Sized > Drop for RwLockReadGuard < '_ , T > { # [inline] fn drop (& mut self) { unsafe { self . lock . read_unlock () ; } } }
};
}
