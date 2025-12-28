macro_rules! deps {
    () => {
        RawMutex!();
        ReentrantMutexGuard!();
        GetThreadId!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < 'a , R : RawMutex + 'a , G : GetThreadId + 'a , T : ? Sized + 'a > Deref for ReentrantMutexGuard < 'a , R , G , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . remutex . data . get () } } }
    };
}

impl_75!();