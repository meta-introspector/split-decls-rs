macro_rules! deps {
    () => {
        YesS3!();
        BSwap!();
        MultiLane!();
        Machine!();
        RotateEachWord32!();
        Vec2!();
        YesS4!();
        Avx2Machine!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl < NI : Copy > u32x4x2 < Avx2Machine < NI > > for u32x4x2_sse2 < YesS3 , YesS4 , NI > where u32x4_sse2 < YesS3 , YesS4 , NI > : RotateEachWord32 + BSwap , Avx2Machine < NI > : Machine , u32x4x2_sse2 < YesS3 , YesS4 , NI > : MultiLane < [< Avx2Machine < NI > as Machine > :: u32x4 ; 2] > , u32x4x2_sse2 < YesS3 , YesS4 , NI > : Vec2 < < Avx2Machine < NI > as Machine > :: u32x4 > , { }
    };
}

impl_182!()