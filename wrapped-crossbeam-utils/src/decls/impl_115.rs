macro_rules! deps {
    () => {
        ShardedLock!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Send > Send for ShardedLock < T > { }
    };
}

impl_115!();