macro_rules! deps {
    () => {
        BitOps64!();
        BitOps0!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < W , G > BitOps64 for x2 < W , G > where W : BitOps64 + BitOps0 , G : Copy , { }
    };
}

impl_10!()