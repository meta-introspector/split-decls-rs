macro_rules! deps {
    () => {
        ShardedLockWriteGuard!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl < T : ? Sized + fmt :: Display > fmt :: Display for ShardedLockWriteGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
    };
}

impl_133!();