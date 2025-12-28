macro_rules! deps {
    () => {
        RawMutex!();
        GetThreadId!();
        MappedReentrantMutexGuard!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl < 'a , R : RawMutex + 'a , G : GetThreadId + 'a , T : ? Sized + 'a > Drop for MappedReentrantMutexGuard < 'a , R , G , T > { # [inline] fn drop (& mut self) { unsafe { self . raw . unlock () ; } } }
    };
}

impl_90!();