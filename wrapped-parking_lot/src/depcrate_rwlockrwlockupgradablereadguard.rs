// Generated macro for RwLockUpgradableReadGuard (type)
macro_rules! Depcrate_rwlockRwLockUpgradableReadGuard {
() => {
// Module: crate::rwlock
// Provides: {"RwLockUpgradableReadGuard"}
// Dependencies: {}
# [doc = " RAII structure used to release the upgradable read access of a lock when"] # [doc = " dropped."] pub type RwLockUpgradableReadGuard < 'a , T > = lock_api :: RwLockUpgradableReadGuard < 'a , RawRwLock , T > ;
};
}
