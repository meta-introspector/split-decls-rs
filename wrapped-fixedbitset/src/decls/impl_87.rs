macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl Default for FixedBitSet { fn default () -> Self { Self :: new () } }
    };
}

impl_87!()