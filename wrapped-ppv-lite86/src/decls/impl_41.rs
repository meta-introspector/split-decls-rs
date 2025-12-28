macro_rules! deps {
    () => {
        BitOps32!();
        BitOps0!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < W > BitOps32 for x4 < W > where W : BitOps32 + BitOps0 { }
    };
}

impl_41!();