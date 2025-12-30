// Generated macro for RwLockReadGuard (struct)
macro_rules! Depcrate_rwlockRwLockReadGuard {
() => {
// Module: crate::rwlock
// Provides: {"RwLockReadGuard"}
// Dependencies: {}
# [doc = " A guard that releases the read lock when dropped."] # [clippy :: has_significant_drop] pub struct RwLockReadGuard < 'a , T : ? Sized > { # [doc = " Reference to underlying locking implementation."] # [doc = " Doesn't depend on `T`."] lock : & 'a RawRwLock , # [doc = " Pointer to the value protected by the lock. Covariant in `T`."] value : * const T , }
};
}
