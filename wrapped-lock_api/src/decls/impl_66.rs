macro_rules! deps {
    () => {
        RawMutex!();
        ReentrantMutex!();
        GetThreadId!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < R : RawMutex , G : GetThreadId , T : ? Sized + Default > Default for ReentrantMutex < R , G , T > { # [inline] fn default () -> ReentrantMutex < R , G , T > { ReentrantMutex :: new (Default :: default ()) } }
    };
}

impl_66!();