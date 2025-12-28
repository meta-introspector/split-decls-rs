macro_rules! deps {
    () => {
        ShardedLock!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Send + Sync > Sync for ShardedLock < T > { }
    };
}

impl_116!();