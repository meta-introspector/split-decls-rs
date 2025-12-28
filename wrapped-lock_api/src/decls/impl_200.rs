macro_rules! deps {
    () => {
        RawRwLock!();
        MappedRwLockWriteGuard!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl < 'a , R : RawRwLock + 'a , T : fmt :: Debug + ? Sized + 'a > fmt :: Debug for MappedRwLockWriteGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
    };
}

impl_200!()