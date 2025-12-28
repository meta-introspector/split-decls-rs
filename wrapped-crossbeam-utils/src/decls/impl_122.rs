macro_rules! deps {
    () => {
        ShardedLock!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < T : Default > Default for ShardedLock < T > { fn default () -> Self { Self :: new (Default :: default ()) } }
    };
}

impl_122!()