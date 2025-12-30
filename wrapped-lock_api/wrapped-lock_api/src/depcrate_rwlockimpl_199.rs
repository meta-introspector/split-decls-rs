// Generated macro for impl_199 (impl)
macro_rules! Depcrate_rwlockimpl_199 {
() => {
// Module: crate::rwlock
// Provides: {"impl_199"}
// Dependencies: {}
impl < 'a , R : RawRwLockUpgrade + 'a , T : ? Sized + 'a > Drop for RwLockUpgradableReadGuard < 'a , R , T > { # [inline] fn drop (& mut self) { unsafe { self . rwlock . raw . unlock_upgradable () ; } } }
};
}
