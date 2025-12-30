// Generated macro for MappedRwLockReadGuard (type)
macro_rules! Depcrate_rwlockMappedRwLockReadGuard {
() => {
// Module: crate::rwlock
// Provides: {"MappedRwLockReadGuard"}
// Dependencies: {}
# [doc = " An RAII read lock guard returned by `RwLockReadGuard::map`, which can point to a"] # [doc = " subfield of the protected data."] # [doc = ""] # [doc = " The main difference between `MappedRwLockReadGuard` and `RwLockReadGuard` is that the"] # [doc = " former doesn't support temporarily unlocking and re-locking, since that"] # [doc = " could introduce soundness issues if the locked object is modified by another"] # [doc = " thread."] pub type MappedRwLockReadGuard < 'a , T > = lock_api :: MappedRwLockReadGuard < 'a , RawRwLock , T > ;
};
}
