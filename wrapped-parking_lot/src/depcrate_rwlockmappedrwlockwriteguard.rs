// Generated macro for MappedRwLockWriteGuard (type)
macro_rules! Depcrate_rwlockMappedRwLockWriteGuard {
() => {
// Module: crate::rwlock
// Provides: {"MappedRwLockWriteGuard"}
// Dependencies: {}
# [doc = " An RAII write lock guard returned by `RwLockWriteGuard::map`, which can point to a"] # [doc = " subfield of the protected data."] # [doc = ""] # [doc = " The main difference between `MappedRwLockWriteGuard` and `RwLockWriteGuard` is that the"] # [doc = " former doesn't support temporarily unlocking and re-locking, since that"] # [doc = " could introduce soundness issues if the locked object is modified by another"] # [doc = " thread."] pub type MappedRwLockWriteGuard < 'a , T > = lock_api :: MappedRwLockWriteGuard < 'a , RawRwLock , T > ;
};
}
