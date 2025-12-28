macro_rules! deps {
    () => {
        GetThreadId!();
        RawMutex!();
        MappedReentrantMutexGuard!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < 'a , R : RawMutex + 'a , G : GetThreadId + 'a , T : ? Sized + 'a > Deref for MappedReentrantMutexGuard < 'a , R , G , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . data } } }
    };
}

impl_89!();