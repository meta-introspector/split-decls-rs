macro_rules! deps {
    () => {
        ArcRwLockWriteGuard!();
        RawRwLock!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawRwLock , T : fmt :: Display + ? Sized > fmt :: Display for ArcRwLockWriteGuard < R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
    };
}

impl_159!()