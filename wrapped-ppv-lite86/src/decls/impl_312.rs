macro_rules! deps {
    () => {
        ArithOps!();
    };
}

macro_rules! impl_312 {
    () => {
        deps!();
        impl ArithOps for u64x2_generic { }
    };
}

impl_312!()