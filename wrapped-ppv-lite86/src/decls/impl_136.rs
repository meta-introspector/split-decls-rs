macro_rules! deps {
    () => {
        BSwap!();
        ArithOps!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl < S3 : Copy , S4 : Copy , NI : Copy > ArithOps for u64x2_sse2 < S3 , S4 , NI > where u64x2_sse2 < S3 , S4 , NI > : BSwap { }
    };
}

impl_136!();