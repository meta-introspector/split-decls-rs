macro_rules! deps {
    () => {
        ShardedLockReadGuard!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Sync > Sync for ShardedLockReadGuard < '_ , T > { }
    };
}

impl_125!()