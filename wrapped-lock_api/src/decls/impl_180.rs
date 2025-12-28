macro_rules! deps {
    () => {
        RawRwLockUpgrade!();
        ArcRwLockUpgradableReadGuard!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawRwLockUpgrade , T : fmt :: Debug + ? Sized > fmt :: Debug for ArcRwLockUpgradableReadGuard < R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
    };
}

impl_180!();