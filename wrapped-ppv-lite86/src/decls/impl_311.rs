macro_rules! deps {
    () => {
        ArithOps!();
    };
}

macro_rules! impl_311 {
    () => {
        deps!();
        impl ArithOps for u32x4_generic { }
    };
}

impl_311!()