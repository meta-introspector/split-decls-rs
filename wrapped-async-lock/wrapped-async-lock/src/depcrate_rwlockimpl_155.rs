// Generated macro for impl_155 (impl)
macro_rules! Depcrate_rwlockimpl_155 {
() => {
// Module: crate::rwlock
// Provides: {"impl_155"}
// Dependencies: {}
impl < T : ? Sized > Drop for RwLockWriteGuard < '_ , T > { # [inline] fn drop (& mut self) { unsafe { self . lock . write_unlock () ; } } }
};
}
