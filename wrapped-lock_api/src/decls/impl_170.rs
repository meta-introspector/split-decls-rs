macro_rules! deps {
    () => {
        RawRwLockUpgrade!();
        RwLockUpgradableReadGuard!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl < 'a , R : RawRwLockUpgrade + 'a , T : fmt :: Display + ? Sized + 'a > fmt :: Display for RwLockUpgradableReadGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
    };
}

impl_170!()