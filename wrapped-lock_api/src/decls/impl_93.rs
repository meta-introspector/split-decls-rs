macro_rules! deps {
    () => {
        MappedReentrantMutexGuard!();
        GetThreadId!();
        RawMutex!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        # [cfg (feature = "owning_ref")] unsafe impl < 'a , R : RawMutex + 'a , G : GetThreadId + 'a , T : ? Sized + 'a > StableAddress for MappedReentrantMutexGuard < 'a , R , G , T > { }
    };
}

impl_93!();