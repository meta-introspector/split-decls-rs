macro_rules! deps {
    () => {
        BSwap!();
        Avx2Machine!();
        YesS3!();
        YesS4!();
        Swap64!();
        RotateEachWord32!();
        Machine!();
        RotateEachWord64!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < NI : Copy > u128x1 < Avx2Machine < NI > > for u128x1_sse2 < YesS3 , YesS4 , NI > where u128x1_sse2 < YesS3 , YesS4 , NI > : Swap64 + RotateEachWord64 + RotateEachWord32 + BSwap , Machine86 < YesS3 , YesS4 , NI > : Machine , u128x1_sse2 < YesS3 , YesS4 , NI > : Into < < Machine86 < YesS3 , YesS4 , NI > as Machine > :: u32x4 > , u128x1_sse2 < YesS3 , YesS4 , NI > : Into < < Machine86 < YesS3 , YesS4 , NI > as Machine > :: u64x2 > , { }
    };
}

impl_146!()