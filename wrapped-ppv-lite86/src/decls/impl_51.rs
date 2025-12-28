macro_rules! deps {
    () => {
        ArithOps!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < W > ArithOps for x4 < W > where W : ArithOps { }
    };
}

impl_51!()