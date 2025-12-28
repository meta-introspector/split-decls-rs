macro_rules! deps {
    () => {
        RawMutex!();
        MutexGuard!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        # [cfg (feature = "owning_ref")] unsafe impl < 'a , R : RawMutex + 'a , T : ? Sized + 'a > StableAddress for MutexGuard < 'a , R , T > { }
    };
}

impl_30!();