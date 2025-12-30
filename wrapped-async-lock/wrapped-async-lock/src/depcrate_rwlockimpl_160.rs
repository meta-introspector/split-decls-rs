// Generated macro for impl_160 (impl)
macro_rules! Depcrate_rwlockimpl_160 {
() => {
// Module: crate::rwlock
// Provides: {"impl_160"}
// Dependencies: {}
impl < T : ? Sized > DerefMut for RwLockWriteGuard < '_ , T > { # [inline] fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . value } } }
};
}
