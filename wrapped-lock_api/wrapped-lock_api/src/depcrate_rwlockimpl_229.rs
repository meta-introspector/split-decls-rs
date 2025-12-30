// Generated macro for impl_229 (impl)
macro_rules! Depcrate_rwlockimpl_229 {
() => {
// Module: crate::rwlock
// Provides: {"impl_229"}
// Dependencies: {}
impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > DerefMut for MappedRwLockWriteGuard < 'a , R , T > { # [inline] fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . data } } }
};
}
