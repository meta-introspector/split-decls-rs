macro_rules! deps {
    () => {
        ShardedLock!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < T : ? Sized > RefUnwindSafe for ShardedLock < T > { }
    };
}

impl_118!()