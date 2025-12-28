macro_rules! deps {
    () => {
        RotateEachWord64!();
        Avx2Machine!();
        RotateEachWord32!();
        YesS4!();
        YesS3!();
        BSwap!();
        MultiLane!();
        Vec2!();
        Machine!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < NI : Copy > u64x2 < Avx2Machine < NI > > for u64x2_sse2 < YesS3 , YesS4 , NI > where u64x2_sse2 < YesS3 , YesS4 , NI > : RotateEachWord64 + RotateEachWord32 + BSwap + MultiLane < [u64 ; 2] > + Vec2 < u64 > , Machine86 < YesS3 , YesS4 , NI > : Machine , { }
    };
}

impl_145!();