macro_rules! deps {
    () => {
        Swap64!();
        MultiLane!();
        YesS4!();
        YesS3!();
        Vec2!();
        Avx2Machine!();
        Words4!();
        RotateEachWord32!();
        Vec4!();
        BSwap!();
        Machine!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        impl < NI : Copy > Machine for Avx2Machine < NI > where sse2 :: u128x1_sse2 < YesS3 , YesS4 , NI > : BSwap + Swap64 , sse2 :: u64x2_sse2 < YesS3 , YesS4 , NI > : BSwap + RotateEachWord32 + MultiLane < [u64 ; 2] > + Vec2 < u64 > , sse2 :: u32x4_sse2 < YesS3 , YesS4 , NI > : BSwap + RotateEachWord32 + MultiLane < [u32 ; 4] > + Vec4 < u32 > , sse2 :: u64x4_sse2 < YesS3 , YesS4 , NI > : BSwap + Words4 , { type u32x4 = sse2 :: u32x4_sse2 < YesS3 , YesS4 , NI > ; type u64x2 = sse2 :: u64x2_sse2 < YesS3 , YesS4 , NI > ; type u128x1 = sse2 :: u128x1_sse2 < YesS3 , YesS4 , NI > ; type u32x4x2 = sse2 :: avx2 :: u32x4x2_avx2 < NI > ; type u64x2x2 = sse2 :: u64x2x2_sse2 < YesS3 , YesS4 , NI > ; type u64x4 = sse2 :: u64x4_sse2 < YesS3 , YesS4 , NI > ; type u128x2 = sse2 :: u128x2_sse2 < YesS3 , YesS4 , NI > ; type u32x4x4 = sse2 :: avx2 :: u32x4x4_avx2 < NI > ; type u64x2x4 = sse2 :: u64x2x4_sse2 < YesS3 , YesS4 , NI > ; type u128x4 = sse2 :: u128x4_sse2 < YesS3 , YesS4 , NI > ; # [inline (always)] unsafe fn instance () -> Self { Avx2Machine (PhantomData) } }
    };
}

impl_219!()