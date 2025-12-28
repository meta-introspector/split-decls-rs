macro_rules! deps {
    () => {
        ReentrantMutexGuard!();
        GetThreadId!();
        RawMutex!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < 'a , R : RawMutex + 'a , G : GetThreadId + 'a , T : ? Sized + 'a > Drop for ReentrantMutexGuard < 'a , R , G , T > { # [inline] fn drop (& mut self) { unsafe { self . remutex . raw . unlock () ; } } }
    };
}

impl_76!()