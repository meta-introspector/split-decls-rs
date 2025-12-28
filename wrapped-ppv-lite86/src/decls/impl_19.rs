macro_rules! deps {
    () => {
        ArithOps!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < W , G > ArithOps for x2 < W , G > where W : ArithOps , G : Copy , { }
    };
}

impl_19!();