macro_rules! deps {
    () => {
        ShardedLockReadGuard!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < T : fmt :: Debug > fmt :: Debug for ShardedLockReadGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ShardedLockReadGuard") . field ("lock" , & self . lock) . finish () } }
    };
}

impl_127!();