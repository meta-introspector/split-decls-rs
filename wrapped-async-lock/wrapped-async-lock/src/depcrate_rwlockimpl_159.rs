// Generated macro for impl_159 (impl)
macro_rules! Depcrate_rwlockimpl_159 {
() => {
// Module: crate::rwlock
// Provides: {"impl_159"}
// Dependencies: {}
impl < T : ? Sized > Deref for RwLockWriteGuard < '_ , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . value } } }
};
}
