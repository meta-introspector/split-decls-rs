// Generated macro for RawRwLockRecursive (trait)
macro_rules! Depcrate_rwlockRawRwLockRecursive {
() => {
// Module: crate::rwlock
// Provides: {"RawRwLockRecursive"}
// Dependencies: {}
# [doc = " Additional methods for `RwLock`s which support recursive read locks."] # [doc = ""] # [doc = " These are guaranteed to succeed without blocking if"] # [doc = " another read lock is held at the time of the call. This allows a thread"] # [doc = " to recursively lock a `RwLock`. However using this method can cause"] # [doc = " writers to starve since readers no longer block if a writer is waiting"] # [doc = " for the lock."] pub unsafe trait RawRwLockRecursive : RawRwLock { # [doc = " Acquires a shared lock without deadlocking in case of a recursive lock."] fn lock_shared_recursive (& self) ; # [doc = " Attempts to acquire a shared lock without deadlocking in case of a recursive lock."] fn try_lock_shared_recursive (& self) -> bool ; }
};
}
