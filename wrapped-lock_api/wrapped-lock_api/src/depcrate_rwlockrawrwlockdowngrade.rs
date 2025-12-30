// Generated macro for RawRwLockDowngrade (trait)
macro_rules! Depcrate_rwlockRawRwLockDowngrade {
() => {
// Module: crate::rwlock
// Provides: {"RawRwLockDowngrade"}
// Dependencies: {}
# [doc = " Additional methods for `RwLock`s which support atomically downgrading an"] # [doc = " exclusive lock to a shared lock."] pub unsafe trait RawRwLockDowngrade : RawRwLock { # [doc = " Atomically downgrades an exclusive lock into a shared lock without"] # [doc = " allowing any thread to take an exclusive lock in the meantime."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method may only be called if an exclusive lock is held in the current context."] unsafe fn downgrade (& self) ; }
};
}
