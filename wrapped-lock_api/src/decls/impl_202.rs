macro_rules! deps {
    () => {
        RawRwLock!();
        MappedRwLockWriteGuard!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        # [cfg (feature = "owning_ref")] unsafe impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > StableAddress for MappedRwLockWriteGuard < 'a , R , T > { }
    };
}

impl_202!();