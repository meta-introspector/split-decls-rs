macro_rules! deps {
    () => {
        MultiLane!();
        Vec4!();
        RotateEachWord32!();
        Machine!();
        BSwap!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < S3 : Copy , S4 : Copy , NI : Copy > u32x4 < Machine86 < S3 , S4 , NI > > for u32x4_sse2 < S3 , S4 , NI > where u32x4_sse2 < S3 , S4 , NI > : RotateEachWord32 + BSwap + MultiLane < [u32 ; 4] > + Vec4 < u32 > , Machine86 < S3 , S4 , NI > : Machine , { }
    };
}

impl_141!();