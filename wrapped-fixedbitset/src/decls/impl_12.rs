macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        unsafe impl Sync for FixedBitSet { }
    };
}

impl_12!()