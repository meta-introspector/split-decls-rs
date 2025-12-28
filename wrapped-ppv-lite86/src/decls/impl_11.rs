macro_rules! deps {
    () => {
        BitOps128!();
        BitOps0!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < W , G > BitOps128 for x2 < W , G > where W : BitOps128 + BitOps0 , G : Copy , { }
    };
}

impl_11!();