macro_rules! deps {
    () => {
        Avx2Machine!();
        BSwap!();
        YesS3!();
        Machine!();
        Vec2!();
        MultiLane!();
        Swap64!();
        YesS4!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl < NI : Copy > u128x2 < Avx2Machine < NI > > for u128x2_sse2 < YesS3 , YesS4 , NI > where u128x1_sse2 < YesS3 , YesS4 , NI > : Swap64 + BSwap , Avx2Machine < NI > : Machine , u128x2_sse2 < YesS3 , YesS4 , NI > : MultiLane < [< Avx2Machine < NI > as Machine > :: u128x1 ; 2] > , u128x2_sse2 < YesS3 , YesS4 , NI > : Vec2 < < Avx2Machine < NI > as Machine > :: u128x1 > , u128x2_sse2 < YesS3 , YesS4 , NI > : Into < < Avx2Machine < NI > as Machine > :: u32x4x2 > , u128x2_sse2 < YesS3 , YesS4 , NI > : Into < < Avx2Machine < NI > as Machine > :: u64x2x2 > , u128x2_sse2 < YesS3 , YesS4 , NI > : Into < < Avx2Machine < NI > as Machine > :: u64x4 > , { }
    };
}

impl_185!()