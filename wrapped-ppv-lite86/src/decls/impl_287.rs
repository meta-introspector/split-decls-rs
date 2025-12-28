macro_rules! deps {
    () => {
        BitOps0!();
    };
}

macro_rules! impl_287 {
    () => {
        deps!();
        impl BitOps0 for u64x2_generic { }
    };
}

impl_287!();