// Generated macro for impl_219 (impl)
macro_rules! Depcrate_rwlockimpl_219 {
() => {
// Module: crate::rwlock
// Provides: {"impl_219"}
// Dependencies: {}
impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > Drop for MappedRwLockReadGuard < 'a , R , T > { # [inline] fn drop (& mut self) { unsafe { self . raw . unlock_shared () ; } } }
};
}
