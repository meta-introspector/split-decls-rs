// Generated macro for RwLockReadGuard (type)
macro_rules! Depcrate_rwlockRwLockReadGuard {
() => {
// Module: crate::rwlock
// Provides: {"RwLockReadGuard"}
// Dependencies: {}
# [doc = " RAII structure used to release the shared read access of a lock when"] # [doc = " dropped."] pub type RwLockReadGuard < 'a , T > = lock_api :: RwLockReadGuard < 'a , RawRwLock , T > ;
};
}
