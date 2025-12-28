macro_rules! deps {
    () => {
        RwLockUpgradableReadGuard!();
        RwLockWriteGuard!();
        RawRwLockUpgradeDowngrade!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < 'a , R : RawRwLockUpgradeDowngrade + 'a , T : ? Sized + 'a > RwLockWriteGuard < 'a , R , T > { # [doc = " Atomically downgrades a write lock into an upgradable read lock without allowing any"] # [doc = " writers to take exclusive access of the lock in the meantime."] # [doc = ""] # [doc = " Note that if there are any writers currently waiting to take the lock"] # [doc = " then other readers may not be able to acquire the lock even if it was"] # [doc = " downgraded."] # [track_caller] pub fn downgrade_to_upgradable (s : Self) -> RwLockUpgradableReadGuard < 'a , R , T > { unsafe { s . rwlock . raw . downgrade_to_upgradable () ; } let rwlock = s . rwlock ; mem :: forget (s) ; RwLockUpgradableReadGuard { rwlock , marker : PhantomData , } } }
    };
}

impl_142!();