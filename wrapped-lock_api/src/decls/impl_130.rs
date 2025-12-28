macro_rules! deps {
    () => {
        RawRwLock!();
        RwLockReadGuard!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        # [cfg (feature = "owning_ref")] unsafe impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > StableAddress for RwLockReadGuard < 'a , R , T > { }
    };
}

impl_130!()