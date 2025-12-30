// Generated macro for impl_169 (impl)
macro_rules! Depcrate_rwlockimpl_169 {
() => {
// Module: crate::rwlock
// Provides: {"impl_169"}
// Dependencies: {}
impl < T : ? Sized > Deref for RwLockWriteGuardArc < T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . lock . value . get () } } }
};
}
