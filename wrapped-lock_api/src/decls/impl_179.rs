macro_rules! deps {
    () => {
        ArcRwLockUpgradableReadGuard!();
        RawRwLockUpgrade!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawRwLockUpgrade , T : ? Sized > Drop for ArcRwLockUpgradableReadGuard < R , T > { # [inline] fn drop (& mut self) { unsafe { self . rwlock . raw . unlock_upgradable () ; } } }
    };
}

impl_179!()