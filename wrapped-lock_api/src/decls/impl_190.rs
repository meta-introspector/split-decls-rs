macro_rules! deps {
    () => {
        RawRwLock!();
        MappedRwLockReadGuard!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl < 'a , R : RawRwLock + 'a , T : fmt :: Display + ? Sized + 'a > fmt :: Display for MappedRwLockReadGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
    };
}

impl_190!()