macro_rules! deps {
    () => {
        RawMutex!();
        GetThreadId!();
        MappedReentrantMutexGuard!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        unsafe impl < 'a , R : RawMutex + Sync + 'a , G : GetThreadId + Sync + 'a , T : ? Sized + Sync + 'a > Sync for MappedReentrantMutexGuard < 'a , R , G , T > { }
    };
}

impl_86!()