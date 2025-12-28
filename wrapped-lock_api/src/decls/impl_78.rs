macro_rules! deps {
    () => {
        RawMutex!();
        GetThreadId!();
        ReentrantMutexGuard!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < 'a , R : RawMutex + 'a , G : GetThreadId + 'a , T : fmt :: Display + ? Sized + 'a > fmt :: Display for ReentrantMutexGuard < 'a , R , G , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
    };
}

impl_78!();