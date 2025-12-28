macro_rules! deps {
    () => {
        RawRwLockUpgrade!();
        RawRwLockFair!();
    };
}

macro_rules! RawRwLockUpgradeFair {
    () => {
        deps!();
        # [doc = " Additional methods for `RwLock`s which support upgradable locks and fair"] # [doc = " unlocking."] pub unsafe trait RawRwLockUpgradeFair : RawRwLockUpgrade + RawRwLockFair { # [doc = " Releases an upgradable lock using a fair unlock protocol."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method may only be called if an upgradable lock is held in the current context."] unsafe fn unlock_upgradable_fair (& self) ; # [doc = " Temporarily yields an upgradable lock to a waiting thread if there is one."] # [doc = ""] # [doc = " This method is functionally equivalent to calling `unlock_upgradable_fair` followed"] # [doc = " by `lock_upgradable`, however it can be much more efficient in the case where there"] # [doc = " are no waiting threads."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method may only be called if an upgradable lock is held in the current context."] unsafe fn bump_upgradable (& self) { self . unlock_upgradable_fair () ; self . lock_upgradable () ; } }
    };
}

RawRwLockUpgradeFair!();