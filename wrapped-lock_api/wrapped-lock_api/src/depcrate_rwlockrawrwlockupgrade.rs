// Generated macro for RawRwLockUpgrade (trait)
macro_rules! Depcrate_rwlockRawRwLockUpgrade {
() => {
// Module: crate::rwlock
// Provides: {"RawRwLockUpgrade"}
// Dependencies: {}
# [doc = " Additional methods for `RwLock`s which support atomically upgrading a shared"] # [doc = " lock to an exclusive lock."] # [doc = ""] # [doc = " This requires acquiring a special \"upgradable read lock\" instead of a"] # [doc = " normal shared lock. There may only be one upgradable lock at any time,"] # [doc = " otherwise deadlocks could occur when upgrading."] pub unsafe trait RawRwLockUpgrade : RawRwLock { # [doc = " Acquires an upgradable lock, blocking the current thread until it is able to do so."] fn lock_upgradable (& self) ; # [doc = " Attempts to acquire an upgradable lock without blocking."] fn try_lock_upgradable (& self) -> bool ; # [doc = " Releases an upgradable lock."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method may only be called if an upgradable lock is held in the current context."] unsafe fn unlock_upgradable (& self) ; # [doc = " Upgrades an upgradable lock to an exclusive lock."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method may only be called if an upgradable lock is held in the current context."] unsafe fn upgrade (& self) ; # [doc = " Attempts to upgrade an upgradable lock to an exclusive lock without"] # [doc = " blocking."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method may only be called if an upgradable lock is held in the current context."] unsafe fn try_upgrade (& self) -> bool ; }
};
}
