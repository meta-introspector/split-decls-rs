macro_rules! deps {
    () => {
        MappedRwLockReadGuard!();
        RawRwLock!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl < 'a , R : RawRwLock + 'a , T : fmt :: Debug + ? Sized + 'a > fmt :: Debug for MappedRwLockReadGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
    };
}

impl_189!()