macro_rules! deps {
    () => {
        ArithOps!();
    };
}

macro_rules! impl_313 {
    () => {
        deps!();
        impl ArithOps for u128x1_generic { }
    };
}

impl_313!();