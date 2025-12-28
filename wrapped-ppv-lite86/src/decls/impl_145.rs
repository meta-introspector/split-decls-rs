macro_rules! deps {
    () => {
        Vec2!();
        Machine!();
        RotateEachWord32!();
        Avx2Machine!();
        YesS4!();
        BSwap!();
        MultiLane!();
        RotateEachWord64!();
        YesS3!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < NI : Copy > u64x2 < Avx2Machine < NI > > for u64x2_sse2 < YesS3 , YesS4 , NI > where u64x2_sse2 < YesS3 , YesS4 , NI > : RotateEachWord64 + RotateEachWord32 + BSwap + MultiLane < [u64 ; 2] > + Vec2 < u64 > , Machine86 < YesS3 , YesS4 , NI > : Machine , { }
    };
}

impl_145!()