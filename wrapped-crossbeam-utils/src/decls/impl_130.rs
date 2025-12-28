macro_rules! deps {
    () => {
        ShardedLockWriteGuard!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Sync > Sync for ShardedLockWriteGuard < '_ , T > { }
    };
}

impl_130!()