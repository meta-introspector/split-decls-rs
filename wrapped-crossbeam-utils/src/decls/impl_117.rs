macro_rules! deps {
    () => {
        ShardedLock!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < T : ? Sized > UnwindSafe for ShardedLock < T > { }
    };
}

impl_117!();