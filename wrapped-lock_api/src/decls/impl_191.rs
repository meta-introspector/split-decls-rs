macro_rules! deps {
    () => {
        MappedRwLockReadGuard!();
        RawRwLock!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        # [cfg (feature = "owning_ref")] unsafe impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > StableAddress for MappedRwLockReadGuard < 'a , R , T > { }
    };
}

impl_191!();