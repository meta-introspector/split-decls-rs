// Generated macro for impl_176 (impl)
macro_rules! Depcrate_rwlockimpl_176 {
() => {
// Module: crate::rwlock
// Provides: {"impl_176"}
// Dependencies: {}
impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > DerefMut for RwLockWriteGuard < 'a , R , T > { # [inline] fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . rwlock . data . get () } } }
};
}
