macro_rules! deps {
    () => {
        GetThreadId!();
        ReentrantMutex!();
        RawMutex!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < R : RawMutex , G : GetThreadId , T > From < T > for ReentrantMutex < R , G , T > { # [inline] fn from (t : T) -> ReentrantMutex < R , G , T > { ReentrantMutex :: new (t) } }
    };
}

impl_67!()