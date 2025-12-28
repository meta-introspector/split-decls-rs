macro_rules! deps {
    () => {
        RawRwLock!();
        ArcRwLockReadGuard!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawRwLock , T : fmt :: Debug + ? Sized > fmt :: Debug for ArcRwLockReadGuard < R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
    };
}

impl_136!()