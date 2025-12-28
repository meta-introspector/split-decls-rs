macro_rules! deps {
    () => {
        RwLockWriteGuard!();
        RawRwLock!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl < 'a , R : RawRwLock + 'a , T : fmt :: Debug + ? Sized + 'a > fmt :: Debug for RwLockWriteGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
    };
}

impl_147!();