macro_rules! deps {
    () => {
        Vec2!();
        Swap64!();
        BSwap!();
        MultiLane!();
        Machine!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl < S3 : Copy , S4 : Copy , NI : Copy > u128x2 < Machine86 < S3 , S4 , NI > > for u128x2_sse2 < S3 , S4 , NI > where u128x1_sse2 < S3 , S4 , NI > : Swap64 + BSwap , Machine86 < S3 , S4 , NI > : Machine , u128x2_sse2 < S3 , S4 , NI > : MultiLane < [< Machine86 < S3 , S4 , NI > as Machine > :: u128x1 ; 2] > , u128x2_sse2 < S3 , S4 , NI > : Vec2 < < Machine86 < S3 , S4 , NI > as Machine > :: u128x1 > , u128x2_sse2 < S3 , S4 , NI > : Into < < Machine86 < S3 , S4 , NI > as Machine > :: u32x4x2 > , u128x2_sse2 < S3 , S4 , NI > : Into < < Machine86 < S3 , S4 , NI > as Machine > :: u64x2x2 > , u128x2_sse2 < S3 , S4 , NI > : Into < < Machine86 < S3 , S4 , NI > as Machine > :: u64x4 > , { }
    };
}

impl_181!()