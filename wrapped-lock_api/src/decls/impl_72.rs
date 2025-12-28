macro_rules! deps {
    () => {
        RawMutex!();
        ReentrantMutexGuard!();
        GetThreadId!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        unsafe impl < 'a , R : RawMutex + Sync + 'a , G : GetThreadId + Sync + 'a , T : ? Sized + Sync + 'a > Sync for ReentrantMutexGuard < 'a , R , G , T > { }
    };
}

impl_72!()