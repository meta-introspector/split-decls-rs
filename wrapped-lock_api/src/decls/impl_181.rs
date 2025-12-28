macro_rules! deps {
    () => {
        ArcRwLockUpgradableReadGuard!();
        RawRwLockUpgrade!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawRwLockUpgrade , T : fmt :: Display + ? Sized > fmt :: Display for ArcRwLockUpgradableReadGuard < R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
    };
}

impl_181!()