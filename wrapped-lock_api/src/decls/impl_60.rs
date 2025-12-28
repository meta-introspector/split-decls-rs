macro_rules! deps {
    () => {
        ReentrantMutex!();
        RawMutex!();
        GetThreadId!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        unsafe impl < R : RawMutex + Sync , G : GetThreadId + Sync , T : ? Sized + Send > Sync for ReentrantMutex < R , G , T > { }
    };
}

impl_60!();