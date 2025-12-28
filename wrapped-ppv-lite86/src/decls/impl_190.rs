macro_rules! deps {
    () => {
        Avx2Machine!();
        BSwap!();
        YesS3!();
        RotateEachWord32!();
        Vec4!();
        RotateEachWord64!();
        Machine!();
        YesS4!();
        MultiLane!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl < NI : Copy > u64x2x4 < Avx2Machine < NI > > for u64x2x4_sse2 < YesS3 , YesS4 , NI > where u64x2_sse2 < YesS3 , YesS4 , NI > : RotateEachWord64 + RotateEachWord32 + BSwap , Avx2Machine < NI > : Machine , u64x2x4_sse2 < YesS3 , YesS4 , NI > : MultiLane < [< Avx2Machine < NI > as Machine > :: u64x2 ; 4] > , u64x2x4_sse2 < YesS3 , YesS4 , NI > : Vec4 < < Avx2Machine < NI > as Machine > :: u64x2 > , { }
    };
}

impl_190!();