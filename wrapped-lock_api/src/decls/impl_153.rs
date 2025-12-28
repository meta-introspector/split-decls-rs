macro_rules! deps {
    () => {
        RawRwLockUpgradeDowngrade!();
        ArcRwLockUpgradableReadGuard!();
        ArcRwLockWriteGuard!();
        RwLockWriteGuard!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawRwLockUpgradeDowngrade , T : ? Sized > ArcRwLockWriteGuard < R , T > { # [doc = " Atomically downgrades a write lock into an upgradable read lock without allowing any"] # [doc = " writers to take exclusive access of the lock in the meantime."] # [doc = ""] # [doc = " This is functionally identical to the `downgrade_to_upgradable` method on [`RwLockWriteGuard`]."] # [track_caller] pub fn downgrade_to_upgradable (s : Self) -> ArcRwLockUpgradableReadGuard < R , T > { unsafe { s . rwlock . raw . downgrade_to_upgradable () ; } let s = ManuallyDrop :: new (s) ; let rwlock = unsafe { ptr :: read (& s . rwlock) } ; ArcRwLockUpgradableReadGuard { rwlock , marker : PhantomData , } } }
    };
}

impl_153!()