macro_rules! deps {
    () => {
        RawRwLock!();
        ArcRwLockWriteGuard!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawRwLock , T : fmt :: Debug + ? Sized > fmt :: Debug for ArcRwLockWriteGuard < R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
    };
}

impl_158!();