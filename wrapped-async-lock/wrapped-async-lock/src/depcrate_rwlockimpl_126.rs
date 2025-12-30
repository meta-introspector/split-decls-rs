// Generated macro for impl_126 (impl)
macro_rules! Depcrate_rwlockimpl_126 {
() => {
// Module: crate::rwlock
// Provides: {"impl_126"}
// Dependencies: {}
impl < T : ? Sized > Deref for RwLockReadGuard < '_ , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . value } } }
};
}
