// Generated macro for RawRwLockUpgradeDowngrade (trait)
macro_rules! Depcrate_rwlockRawRwLockUpgradeDowngrade {
() => {
// Module: crate::rwlock
// Provides: {"RawRwLockUpgradeDowngrade"}
// Dependencies: {}
# [doc = " Additional methods for `RwLock`s which support upgradable locks and lock"] # [doc = " downgrading."] pub unsafe trait RawRwLockUpgradeDowngrade : RawRwLockUpgrade + RawRwLockDowngrade { # [doc = " Downgrades an upgradable lock to a shared lock."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method may only be called if an upgradable lock is held in the current context."] unsafe fn downgrade_upgradable (& self) ; # [doc = " Downgrades an exclusive lock to an upgradable lock."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method may only be called if an exclusive lock is held in the current context."] unsafe fn downgrade_to_upgradable (& self) ; }
};
}
