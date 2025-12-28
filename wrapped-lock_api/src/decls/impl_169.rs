macro_rules! deps {
    () => {
        RwLockUpgradableReadGuard!();
        RawRwLockUpgrade!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < 'a , R : RawRwLockUpgrade + 'a , T : fmt :: Debug + ? Sized + 'a > fmt :: Debug for RwLockUpgradableReadGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
    };
}

impl_169!();