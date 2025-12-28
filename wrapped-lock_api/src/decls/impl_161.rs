macro_rules! deps {
    () => {
        RawRwLockUpgrade!();
        RwLockUpgradableReadGuard!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        unsafe impl < 'a , R : RawRwLockUpgrade + 'a , T : ? Sized + Sync + 'a > Sync for RwLockUpgradableReadGuard < 'a , R , T > { }
    };
}

impl_161!();