macro_rules! deps {
    () => {
        BSwap!();
        YesS3!();
        Swap64!();
        YesS4!();
        MultiLane!();
        Vec4!();
        Avx2Machine!();
        Machine!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl < NI : Copy > u128x4 < Avx2Machine < NI > > for u128x4_sse2 < YesS3 , YesS4 , NI > where u128x1_sse2 < YesS3 , YesS4 , NI > : Swap64 + BSwap , Avx2Machine < NI > : Machine , u128x4_sse2 < YesS3 , YesS4 , NI > : MultiLane < [< Avx2Machine < NI > as Machine > :: u128x1 ; 4] > , u128x4_sse2 < YesS3 , YesS4 , NI > : Vec4 < < Avx2Machine < NI > as Machine > :: u128x1 > , u128x4_sse2 < YesS3 , YesS4 , NI > : Into < < Avx2Machine < NI > as Machine > :: u32x4x4 > , u128x4_sse2 < YesS3 , YesS4 , NI > : Into < < Avx2Machine < NI > as Machine > :: u64x2x4 > , { }
    };
}

impl_191!()