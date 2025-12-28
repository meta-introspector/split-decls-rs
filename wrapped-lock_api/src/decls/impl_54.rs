macro_rules! deps {
    () => {
        RawMutex!();
        RawReentrantMutex!();
        GetThreadId!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        unsafe impl < R : RawMutex + Sync , G : GetThreadId + Sync > Sync for RawReentrantMutex < R , G > { }
    };
}

impl_54!()