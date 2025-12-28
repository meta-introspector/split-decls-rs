macro_rules! deps {
    () => {
        RawRwLock!();
        ArcRwLockReadGuard!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawRwLock , T : fmt :: Display + ? Sized > fmt :: Display for ArcRwLockReadGuard < R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
    };
}

impl_137!()