// Generated macro for RwLockWriteGuard (struct)
macro_rules! Depcrate_rwlockRwLockWriteGuard {
() => {
// Module: crate::rwlock
// Provides: {"RwLockWriteGuard"}
// Dependencies: {}
# [doc = " A guard that releases the write lock when dropped."] # [clippy :: has_significant_drop] pub struct RwLockWriteGuard < 'a , T : ? Sized > { # [doc = " Reference to underlying locking implementation."] # [doc = " Doesn't depend on `T`."] # [doc = " This guard holds a lock on the witer mutex!"] lock : & 'a RawRwLock , # [doc = " Pointer to the value protected by the lock. Invariant in `T`."] value : * mut T , }
};
}
