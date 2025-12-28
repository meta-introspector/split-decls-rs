macro_rules! deps {
    () => {
        YesS3!();
        RotateEachWord32!();
        RotateEachWord64!();
        YesS4!();
        Avx2Machine!();
        BSwap!();
        Vec4!();
        MultiLane!();
        Machine!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl < NI : Copy > u64x2x4 < Avx2Machine < NI > > for u64x2x4_sse2 < YesS3 , YesS4 , NI > where u64x2_sse2 < YesS3 , YesS4 , NI > : RotateEachWord64 + RotateEachWord32 + BSwap , Avx2Machine < NI > : Machine , u64x2x4_sse2 < YesS3 , YesS4 , NI > : MultiLane < [< Avx2Machine < NI > as Machine > :: u64x2 ; 4] > , u64x2x4_sse2 < YesS3 , YesS4 , NI > : Vec4 < < Avx2Machine < NI > as Machine > :: u64x2 > , { }
    };
}

impl_190!()