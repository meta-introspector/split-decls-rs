// Generated macro for RwLockUpgradableReadGuard (struct)
macro_rules! Depcrate_rwlockRwLockUpgradableReadGuard {
() => {
// Module: crate::rwlock
// Provides: {"RwLockUpgradableReadGuard"}
// Dependencies: {}
# [doc = " A guard that releases the upgradable read lock when dropped."] # [clippy :: has_significant_drop] pub struct RwLockUpgradableReadGuard < 'a , T : ? Sized > { # [doc = " Reference to underlying locking implementation."] # [doc = " Doesn't depend on `T`."] # [doc = " This guard holds a lock on the witer mutex!"] lock : & 'a RawRwLock , # [doc = " Pointer to the value protected by the lock. Invariant in `T`"] # [doc = " as the upgradable lock could provide write access."] value : * mut T , }
};
}
