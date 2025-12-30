// Generated macro for RwLockReadGuard (struct)
macro_rules! Depcrate_sync_rwlockRwLockReadGuard {
() => {
// Module: crate::sync::rwlock
// Provides: {"RwLockReadGuard"}
// Dependencies: {}
# [doc = " Mock implementation of `std::sync::RwLockReadGuard`"] # [derive (Debug)] pub struct RwLockReadGuard < 'a , T > { lock : & 'a RwLock < T > , data : Option < std :: sync :: RwLockReadGuard < 'a , T > > , }
};
}
