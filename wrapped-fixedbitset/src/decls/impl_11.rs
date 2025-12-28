macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        unsafe impl Send for FixedBitSet { }
    };
}

impl_11!()