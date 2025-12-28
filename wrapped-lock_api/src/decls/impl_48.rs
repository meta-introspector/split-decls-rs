macro_rules! deps {
    () => {
        RawMutex!();
        MappedMutexGuard!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < 'a , R : RawMutex + 'a , T : fmt :: Display + ? Sized + 'a > fmt :: Display for MappedMutexGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
    };
}

impl_48!();