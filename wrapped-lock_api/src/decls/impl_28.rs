macro_rules! deps {
    () => {
        RawMutex!();
        MutexGuard!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < 'a , R : RawMutex + 'a , T : fmt :: Debug + ? Sized + 'a > fmt :: Debug for MutexGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
    };
}

impl_28!()