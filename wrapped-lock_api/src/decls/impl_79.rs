macro_rules! deps {
    () => {
        RawMutex!();
        GetThreadId!();
        ReentrantMutexGuard!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        # [cfg (feature = "owning_ref")] unsafe impl < 'a , R : RawMutex + 'a , G : GetThreadId + 'a , T : ? Sized + 'a > StableAddress for ReentrantMutexGuard < 'a , R , G , T > { }
    };
}

impl_79!();