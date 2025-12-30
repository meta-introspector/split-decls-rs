// Generated macro for impl_158 (impl)
macro_rules! Depcrate_rwlockimpl_158 {
() => {
// Module: crate::rwlock
// Provides: {"impl_158"}
// Dependencies: {}
impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > Drop for RwLockReadGuard < 'a , R , T > { # [inline] fn drop (& mut self) { unsafe { self . rwlock . raw . unlock_shared () ; } } }
};
}
