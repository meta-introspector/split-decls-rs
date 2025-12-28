macro_rules! deps {
    () => {
        RotateEachWord32!();
        RotateEachWord64!();
        BSwap!();
        Machine!();
        Swap64!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < S3 : Copy , S4 : Copy , NI : Copy > u128x1 < Machine86 < S3 , S4 , NI > > for u128x1_sse2 < S3 , S4 , NI > where u128x1_sse2 < S3 , S4 , NI > : Swap64 + RotateEachWord64 + RotateEachWord32 + BSwap , Machine86 < S3 , S4 , NI > : Machine , u128x1_sse2 < S3 , S4 , NI > : Into < < Machine86 < S3 , S4 , NI > as Machine > :: u32x4 > , u128x1_sse2 < S3 , S4 , NI > : Into < < Machine86 < S3 , S4 , NI > as Machine > :: u64x2 > , { }
    };
}

impl_143!();