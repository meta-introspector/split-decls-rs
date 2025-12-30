// Generated macro for impl_170 (impl)
macro_rules! Depcrate_rwlockimpl_170 {
() => {
// Module: crate::rwlock
// Provides: {"impl_170"}
// Dependencies: {}
impl < T : ? Sized > DerefMut for RwLockWriteGuardArc < T > { # [inline] fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . lock . value . get () } } }
};
}
