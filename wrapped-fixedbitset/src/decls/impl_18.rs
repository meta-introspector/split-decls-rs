macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl Default for FixedBitSet { fn default () -> Self { Self :: new () } }
    };
}

impl_18!()