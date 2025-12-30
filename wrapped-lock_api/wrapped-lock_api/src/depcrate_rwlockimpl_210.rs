// Generated macro for impl_210 (impl)
macro_rules! Depcrate_rwlockimpl_210 {
() => {
// Module: crate::rwlock
// Provides: {"impl_210"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] impl < R : RawRwLockUpgrade , T : ? Sized > Drop for ArcRwLockUpgradableReadGuard < R , T > { # [inline] fn drop (& mut self) { unsafe { self . rwlock . raw . unlock_upgradable () ; } } }
};
}
