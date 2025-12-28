macro_rules! deps {
    () => {
        RawRwLock!();
        RwLockWriteGuard!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        # [cfg (feature = "owning_ref")] unsafe impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > StableAddress for RwLockWriteGuard < 'a , R , T > { }
    };
}

impl_149!();