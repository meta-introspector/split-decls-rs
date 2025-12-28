macro_rules! deps {
    () => {
        Vec2!();
        BSwap!();
        YesS3!();
        RotateEachWord32!();
        Machine!();
        Avx2Machine!();
        YesS4!();
        RotateEachWord64!();
        MultiLane!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl < NI : Copy > u64x2x2 < Avx2Machine < NI > > for u64x2x2_sse2 < YesS3 , YesS4 , NI > where u64x2_sse2 < YesS3 , YesS4 , NI > : RotateEachWord64 + RotateEachWord32 + BSwap , Avx2Machine < NI > : Machine , u64x2x2_sse2 < YesS3 , YesS4 , NI > : MultiLane < [< Avx2Machine < NI > as Machine > :: u64x2 ; 2] > , u64x2x2_sse2 < YesS3 , YesS4 , NI > : Vec2 < < Avx2Machine < NI > as Machine > :: u64x2 > , { }
    };
}

impl_183!()