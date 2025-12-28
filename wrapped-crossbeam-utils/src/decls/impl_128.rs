macro_rules! deps {
    () => {
        ShardedLockReadGuard!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < T : ? Sized + fmt :: Display > fmt :: Display for ShardedLockReadGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
    };
}

impl_128!()