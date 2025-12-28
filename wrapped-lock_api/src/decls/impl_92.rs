macro_rules! deps {
    () => {
        MappedReentrantMutexGuard!();
        GetThreadId!();
        RawMutex!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < 'a , R : RawMutex + 'a , G : GetThreadId + 'a , T : fmt :: Display + ? Sized + 'a > fmt :: Display for MappedReentrantMutexGuard < 'a , R , G , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
    };
}

impl_92!();