macro_rules! deps {
    () => {
        RawMutex!();
        MutexGuard!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < 'a , R : RawMutex + 'a , T : fmt :: Display + ? Sized + 'a > fmt :: Display for MutexGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
    };
}

impl_29!();