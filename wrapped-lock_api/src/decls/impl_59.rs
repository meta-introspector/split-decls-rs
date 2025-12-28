macro_rules! deps {
    () => {
        RawMutex!();
        ReentrantMutex!();
        GetThreadId!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        unsafe impl < R : RawMutex + Send , G : GetThreadId + Send , T : ? Sized + Send > Send for ReentrantMutex < R , G , T > { }
    };
}

impl_59!()