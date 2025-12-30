// Generated macro for RwLockWriteGuard (struct)
macro_rules! Depcrate_sync_rwlockRwLockWriteGuard {
() => {
// Module: crate::sync::rwlock
// Provides: {"RwLockWriteGuard"}
// Dependencies: {}
# [doc = " Mock implementation of `std::sync::rwLockWriteGuard`"] # [derive (Debug)] pub struct RwLockWriteGuard < 'a , T > { lock : & 'a RwLock < T > , # [doc = " `data` is an Option so that the Drop impl can drop the std guard and release the std lock"] # [doc = " before releasing the loom mock lock, as that might cause another thread to acquire the lock"] data : Option < std :: sync :: RwLockWriteGuard < 'a , T > > , }
};
}
