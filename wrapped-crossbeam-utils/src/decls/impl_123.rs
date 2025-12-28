macro_rules! deps {
    () => {
        ShardedLock!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < T > From < T > for ShardedLock < T > { fn from (t : T) -> Self { Self :: new (t) } }
    };
}

impl_123!()