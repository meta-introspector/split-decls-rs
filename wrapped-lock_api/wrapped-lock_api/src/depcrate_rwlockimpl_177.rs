// Generated macro for impl_177 (impl)
macro_rules! Depcrate_rwlockimpl_177 {
() => {
// Module: crate::rwlock
// Provides: {"impl_177"}
// Dependencies: {}
impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > Drop for RwLockWriteGuard < 'a , R , T > { # [inline] fn drop (& mut self) { unsafe { self . rwlock . raw . unlock_exclusive () ; } } }
};
}
