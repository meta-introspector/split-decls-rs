// Generated macro for impl_149 (impl)
macro_rules! Depcrate_rwlockimpl_149 {
() => {
// Module: crate::rwlock
// Provides: {"impl_149"}
// Dependencies: {}
impl < T : ? Sized > Deref for RwLockUpgradableReadGuardArc < T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . lock . value . get () } } }
};
}
