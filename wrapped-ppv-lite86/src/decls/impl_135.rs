macro_rules! deps {
    () => {
        ArithOps!();
        BSwap!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl < S3 : Copy , S4 : Copy , NI : Copy > ArithOps for u32x4_sse2 < S3 , S4 , NI > where u32x4_sse2 < S3 , S4 , NI > : BSwap { }
    };
}

impl_135!()