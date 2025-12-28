macro_rules! deps {
    () => {
        MappedRwLockWriteGuard!();
        RawRwLock!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl < 'a , R : RawRwLock + 'a , T : fmt :: Display + ? Sized + 'a > fmt :: Display for MappedRwLockWriteGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
    };
}

impl_201!();