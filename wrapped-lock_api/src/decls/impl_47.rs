macro_rules! deps {
    () => {
        MappedMutexGuard!();
        RawMutex!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < 'a , R : RawMutex + 'a , T : fmt :: Debug + ? Sized + 'a > fmt :: Debug for MappedMutexGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
    };
}

impl_47!();