// Generated macro for impl_144 (impl)
macro_rules! Depcrate_rwlockimpl_144 {
() => {
// Module: crate::rwlock
// Provides: {"impl_144"}
// Dependencies: {}
impl < T : ? Sized > Drop for RwLockUpgradableReadGuardArc < T > { # [inline] fn drop (& mut self) { unsafe { self . lock . raw . upgradable_read_unlock () ; } } }
};
}
