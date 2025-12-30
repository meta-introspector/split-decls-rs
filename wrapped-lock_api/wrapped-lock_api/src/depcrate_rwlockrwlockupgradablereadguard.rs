// Generated macro for RwLockUpgradableReadGuard (struct)
macro_rules! Depcrate_rwlockRwLockUpgradableReadGuard {
() => {
// Module: crate::rwlock
// Provides: {"RwLockUpgradableReadGuard"}
// Dependencies: {}
# [doc = " RAII structure used to release the upgradable read access of a lock when"] # [doc = " dropped."] # [clippy :: has_significant_drop] # [must_use = "if unused the RwLock will immediately unlock"] pub struct RwLockUpgradableReadGuard < 'a , R : RawRwLockUpgrade , T : ? Sized > { rwlock : & 'a RwLock < R , T > , marker : PhantomData < (& 'a T , R :: GuardMarker) > , }
};
}
