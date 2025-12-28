macro_rules! deps {
    () => {
        BSwap!();
        MultiLane!();
        Machine!();
        RotateEachWord32!();
        Vec2!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < S3 : Copy , S4 : Copy , NI : Copy > u32x4x2 < Machine86 < S3 , S4 , NI > > for u32x4x2_sse2 < S3 , S4 , NI > where u32x4_sse2 < S3 , S4 , NI > : RotateEachWord32 + BSwap , Machine86 < S3 , S4 , NI > : Machine , u32x4x2_sse2 < S3 , S4 , NI > : MultiLane < [< Machine86 < S3 , S4 , NI > as Machine > :: u32x4 ; 2] > , u32x4x2_sse2 < S3 , S4 , NI > : Vec2 < < Machine86 < S3 , S4 , NI > as Machine > :: u32x4 > , { }
    };
}

impl_178!()