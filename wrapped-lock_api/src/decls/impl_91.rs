macro_rules! deps {
    () => {
        GetThreadId!();
        MappedReentrantMutexGuard!();
        RawMutex!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl < 'a , R : RawMutex + 'a , G : GetThreadId + 'a , T : fmt :: Debug + ? Sized + 'a > fmt :: Debug for MappedReentrantMutexGuard < 'a , R , G , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
    };
}

impl_91!()