macro_rules! deps {
    () => {
        RawRwLockUpgradeDowngrade!();
        RwLockUpgradableReadGuard!();
        RawRwLockUpgradeTimed!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < 'a , R : RawRwLockUpgradeTimed + RawRwLockUpgradeDowngrade + 'a , T : ? Sized + 'a > RwLockUpgradableReadGuard < 'a , R , T > { # [doc = " Tries to atomically upgrade an upgradable read lock into an exclusive"] # [doc = " write lock, until a timeout is reached."] # [doc = ""] # [doc = " If the access could not be granted before the timeout expires, then"] # [doc = " `None` is returned."] # [doc = ""] # [doc = " Otherwise, calls the provided closure with an exclusive reference to the lock's data,"] # [doc = " and finally downgrades the lock back to an upgradable read lock."] # [doc = " The closure's return value is wrapped in `Some` and returned."] # [doc = ""] # [doc = " This function only requires a mutable reference to the guard, unlike"] # [doc = " `try_upgrade_for` which takes the guard by value."] # [track_caller] pub fn try_with_upgraded_for < Ret , F : FnOnce (& mut T) -> Ret > (& mut self , timeout : R :: Duration , f : F ,) -> Option < Ret > { if unsafe { self . rwlock . raw . try_upgrade_for (timeout) } { defer ! (unsafe { self . rwlock . raw . downgrade_to_upgradable () }) ; Some (f (unsafe { & mut * self . rwlock . data . get () })) } else { None } } # [doc = " Tries to atomically upgrade an upgradable read lock into an exclusive"] # [doc = " write lock, until a timeout is reached."] # [doc = ""] # [doc = " If the access could not be granted before the timeout expires, then"] # [doc = " `None` is returned."] # [doc = ""] # [doc = " Otherwise, calls the provided closure with an exclusive reference to the lock's data,"] # [doc = " and finally downgrades the lock back to an upgradable read lock."] # [doc = " The closure's return value is wrapped in `Some` and returned."] # [doc = ""] # [doc = " This function only requires a mutable reference to the guard, unlike"] # [doc = " `try_upgrade_until` which takes the guard by value."] # [track_caller] pub fn try_with_upgraded_until < Ret , F : FnOnce (& mut T) -> Ret > (& mut self , timeout : R :: Instant , f : F ,) -> Option < Ret > { if unsafe { self . rwlock . raw . try_upgrade_until (timeout) } { defer ! (unsafe { self . rwlock . raw . downgrade_to_upgradable () }) ; Some (f (unsafe { & mut * self . rwlock . data . get () })) } else { None } } }
    };
}

impl_166!();