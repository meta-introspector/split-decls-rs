macro_rules! deps {
    () => {
        RawRwLockUpgrade!();
        RwLockUpgradableReadGuard!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < 'a , R : RawRwLockUpgrade + 'a , T : ? Sized + 'a > Drop for RwLockUpgradableReadGuard < 'a , R , T > { # [inline] fn drop (& mut self) { unsafe { self . rwlock . raw . unlock_upgradable () ; } } }
    };
}

impl_168!()