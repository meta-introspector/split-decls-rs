// Generated macro for RwLockUpgradableReadGuardArc (struct)
macro_rules! Depcrate_rwlockRwLockUpgradableReadGuardArc {
() => {
// Module: crate::rwlock
// Provides: {"RwLockUpgradableReadGuardArc"}
// Dependencies: {}
# [doc = " An owned, reference-counting guard that releases the upgradable read lock when dropped."] # [clippy :: has_significant_drop] pub struct RwLockUpgradableReadGuardArc < T : ? Sized > { # [doc = " We want invariance, so no need for pointer tricks."] lock : Arc < RwLock < T > > , }
};
}
