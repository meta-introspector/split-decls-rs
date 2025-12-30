// Generated macro for RwLockWriteGuard (type)
macro_rules! Depcrate_rwlockRwLockWriteGuard {
() => {
// Module: crate::rwlock
// Provides: {"RwLockWriteGuard"}
// Dependencies: {}
# [doc = " RAII structure used to release the exclusive write access of a lock when"] # [doc = " dropped."] pub type RwLockWriteGuard < 'a , T > = lock_api :: RwLockWriteGuard < 'a , RawRwLock , T > ;
};
}
