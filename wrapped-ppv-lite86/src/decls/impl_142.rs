macro_rules! deps {
    () => {
        BSwap!();
        Machine!();
        RotateEachWord64!();
        Vec2!();
        RotateEachWord32!();
        MultiLane!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < S3 : Copy , S4 : Copy , NI : Copy > u64x2 < Machine86 < S3 , S4 , NI > > for u64x2_sse2 < S3 , S4 , NI > where u64x2_sse2 < S3 , S4 , NI > : RotateEachWord64 + RotateEachWord32 + BSwap + MultiLane < [u64 ; 2] > + Vec2 < u64 > , Machine86 < S3 , S4 , NI > : Machine , { }
    };
}

impl_142!();