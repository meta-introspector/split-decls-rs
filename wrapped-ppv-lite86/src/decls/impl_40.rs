macro_rules! deps {
    () => {
        BitOps0!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < W > BitOps0 for x4 < W > where W : BitOps0 { }
    };
}

impl_40!()