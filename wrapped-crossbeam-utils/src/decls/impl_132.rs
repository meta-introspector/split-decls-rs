macro_rules! deps {
    () => {
        ShardedLockWriteGuard!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < T : fmt :: Debug > fmt :: Debug for ShardedLockWriteGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ShardedLockWriteGuard") . field ("lock" , & self . lock) . finish () } }
    };
}

impl_132!();