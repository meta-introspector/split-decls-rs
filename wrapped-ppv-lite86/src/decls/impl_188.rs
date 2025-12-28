macro_rules! deps {
    () => {
        Vec4!();
        RotateEachWord32!();
        RotateEachWord64!();
        BSwap!();
        MultiLane!();
        Machine!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl < S3 : Copy , S4 : Copy , NI : Copy > u64x2x4 < Machine86 < S3 , S4 , NI > > for u64x2x4_sse2 < S3 , S4 , NI > where u64x2_sse2 < S3 , S4 , NI > : RotateEachWord64 + RotateEachWord32 + BSwap , Machine86 < S3 , S4 , NI > : Machine , u64x2x4_sse2 < S3 , S4 , NI > : MultiLane < [< Machine86 < S3 , S4 , NI > as Machine > :: u64x2 ; 4] > , u64x2x4_sse2 < S3 , S4 , NI > : Vec4 < < Machine86 < S3 , S4 , NI > as Machine > :: u64x2 > , { }
    };
}

impl_188!();