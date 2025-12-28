macro_rules! deps {
    () => {
        BSwap!();
        MultiLane!();
        Swap64!();
        Machine!();
        Vec4!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl < S3 : Copy , S4 : Copy , NI : Copy > u128x4 < Machine86 < S3 , S4 , NI > > for u128x4_sse2 < S3 , S4 , NI > where u128x1_sse2 < S3 , S4 , NI > : Swap64 + BSwap , Machine86 < S3 , S4 , NI > : Machine , u128x4_sse2 < S3 , S4 , NI > : MultiLane < [< Machine86 < S3 , S4 , NI > as Machine > :: u128x1 ; 4] > , u128x4_sse2 < S3 , S4 , NI > : Vec4 < < Machine86 < S3 , S4 , NI > as Machine > :: u128x1 > , u128x4_sse2 < S3 , S4 , NI > : Into < < Machine86 < S3 , S4 , NI > as Machine > :: u32x4x4 > , u128x4_sse2 < S3 , S4 , NI > : Into < < Machine86 < S3 , S4 , NI > as Machine > :: u64x2x4 > , { }
    };
}

impl_189!()