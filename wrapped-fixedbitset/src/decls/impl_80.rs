macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        unsafe impl Send for FixedBitSet { }
    };
}

impl_80!()