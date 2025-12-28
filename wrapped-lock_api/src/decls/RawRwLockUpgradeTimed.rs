macro_rules! deps {
    () => {
        RawRwLockUpgrade!();
        RawRwLockTimed!();
    };
}

macro_rules! RawRwLockUpgradeTimed {
    () => {
        deps!();
        # [doc = " Additional methods for `RwLock`s which support upgradable locks and locking"] # [doc = " with timeouts."] pub unsafe trait RawRwLockUpgradeTimed : RawRwLockUpgrade + RawRwLockTimed { # [doc = " Attempts to acquire an upgradable lock until a timeout is reached."] fn try_lock_upgradable_for (& self , timeout : Self :: Duration) -> bool ; # [doc = " Attempts to acquire an upgradable lock until a timeout is reached."] fn try_lock_upgradable_until (& self , timeout : Self :: Instant) -> bool ; # [doc = " Attempts to upgrade an upgradable lock to an exclusive lock until a"] # [doc = " timeout is reached."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method may only be called if an upgradable lock is held in the current context."] unsafe fn try_upgrade_for (& self , timeout : Self :: Duration) -> bool ; # [doc = " Attempts to upgrade an upgradable lock to an exclusive lock until a"] # [doc = " timeout is reached."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method may only be called if an upgradable lock is held in the current context."] unsafe fn try_upgrade_until (& self , timeout : Self :: Instant) -> bool ; }
    };
}

RawRwLockUpgradeTimed!();