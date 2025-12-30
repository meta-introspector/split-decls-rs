// Generated macro for impl_142 (impl)
macro_rules! Depcrate_rwlockimpl_142 {
() => {
// Module: crate::rwlock
// Provides: {"impl_142"}
// Dependencies: {}
impl < T : ? Sized > Deref for RwLockUpgradableReadGuard < '_ , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . value } } }
};
}
