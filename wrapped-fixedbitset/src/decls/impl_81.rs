macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        unsafe impl Sync for FixedBitSet { }
    };
}

impl_81!();