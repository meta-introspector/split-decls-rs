macro_rules! deps {
    () => {
        RawMutex!();
        MappedMutexGuard!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        # [cfg (feature = "owning_ref")] unsafe impl < 'a , R : RawMutex + 'a , T : ? Sized + 'a > StableAddress for MappedMutexGuard < 'a , R , T > { }
    };
}

impl_49!();