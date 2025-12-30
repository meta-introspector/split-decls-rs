// Generated macro for impl_230 (impl)
macro_rules! Depcrate_rwlockimpl_230 {
() => {
// Module: crate::rwlock
// Provides: {"impl_230"}
// Dependencies: {}
impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > Drop for MappedRwLockWriteGuard < 'a , R , T > { # [inline] fn drop (& mut self) { unsafe { self . raw . unlock_exclusive () ; } } }
};
}
