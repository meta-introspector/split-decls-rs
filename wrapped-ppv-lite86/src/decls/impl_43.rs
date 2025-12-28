macro_rules! deps {
    () => {
        BitOps0!();
        BitOps128!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < W > BitOps128 for x4 < W > where W : BitOps128 + BitOps0 { }
    };
}

impl_43!();