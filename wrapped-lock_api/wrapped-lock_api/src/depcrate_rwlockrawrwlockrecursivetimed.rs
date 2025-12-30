// Generated macro for RawRwLockRecursiveTimed (trait)
macro_rules! Depcrate_rwlockRawRwLockRecursiveTimed {
() => {
// Module: crate::rwlock
// Provides: {"RawRwLockRecursiveTimed"}
// Dependencies: {}
# [doc = " Additional methods for `RwLock`s which support recursive read locks and timeouts."] pub unsafe trait RawRwLockRecursiveTimed : RawRwLockRecursive + RawRwLockTimed { # [doc = " Attempts to acquire a shared lock until a timeout is reached, without"] # [doc = " deadlocking in case of a recursive lock."] fn try_lock_shared_recursive_for (& self , timeout : Self :: Duration) -> bool ; # [doc = " Attempts to acquire a shared lock until a timeout is reached, without"] # [doc = " deadlocking in case of a recursive lock."] fn try_lock_shared_recursive_until (& self , timeout : Self :: Instant) -> bool ; }
};
}
