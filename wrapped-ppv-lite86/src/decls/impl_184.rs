macro_rules! deps {
    () => {
        YesS4!();
        RotateEachWord64!();
        BSwap!();
        YesS3!();
        Vec4!();
        Words4!();
        Avx2Machine!();
        RotateEachWord32!();
        Machine!();
        MultiLane!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl < NI : Copy > u64x4 < Avx2Machine < NI > > for u64x4_sse2 < YesS3 , YesS4 , NI > where u64x2_sse2 < YesS3 , YesS4 , NI > : RotateEachWord64 + RotateEachWord32 + BSwap , Avx2Machine < NI > : Machine , u64x4_sse2 < YesS3 , YesS4 , NI > : MultiLane < [u64 ; 4] > + Vec4 < u64 > + Words4 , { }
    };
}

impl_184!();