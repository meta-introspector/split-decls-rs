// Generated macro for impl_136 (impl)
macro_rules! Depcrate_rwlockimpl_136 {
() => {
// Module: crate::rwlock
// Provides: {"impl_136"}
// Dependencies: {}
impl < T : ? Sized > Drop for RwLockUpgradableReadGuard < '_ , T > { # [inline] fn drop (& mut self) { unsafe { self . lock . upgradable_read_unlock () ; } } }
};
}
