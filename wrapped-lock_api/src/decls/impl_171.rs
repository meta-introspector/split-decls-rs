macro_rules! deps {
    () => {
        RawRwLockUpgrade!();
        RwLockUpgradableReadGuard!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        # [cfg (feature = "owning_ref")] unsafe impl < 'a , R : RawRwLockUpgrade + 'a , T : ? Sized + 'a > StableAddress for RwLockUpgradableReadGuard < 'a , R , T > { }
    };
}

impl_171!()