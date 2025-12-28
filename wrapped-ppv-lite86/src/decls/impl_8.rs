macro_rules! deps {
    () => {
        BitOps0!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < W , G > BitOps0 for x2 < W , G > where W : BitOps0 , G : Copy , { }
    };
}

impl_8!();