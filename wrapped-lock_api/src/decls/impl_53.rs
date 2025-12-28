macro_rules! deps {
    () => {
        RawReentrantMutex!();
        RawMutex!();
        GetThreadId!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        unsafe impl < R : RawMutex + Send , G : GetThreadId + Send > Send for RawReentrantMutex < R , G > { }
    };
}

impl_53!();