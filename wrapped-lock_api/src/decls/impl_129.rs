macro_rules! deps {
    () => {
        RwLockReadGuard!();
        RawRwLock!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < 'a , R : RawRwLock + 'a , T : fmt :: Display + ? Sized + 'a > fmt :: Display for RwLockReadGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
    };
}

impl_129!()