macro_rules! deps {
    () => {
        BitOps32!();
        BitOps0!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < W , G > BitOps32 for x2 < W , G > where W : BitOps32 + BitOps0 , G : Copy , { }
    };
}

impl_9!()