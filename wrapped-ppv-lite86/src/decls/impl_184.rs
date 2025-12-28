macro_rules! deps {
    () => {
        MultiLane!();
        BSwap!();
        YesS3!();
        Vec4!();
        Avx2Machine!();
        Machine!();
        Words4!();
        YesS4!();
        RotateEachWord32!();
        RotateEachWord64!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl < NI : Copy > u64x4 < Avx2Machine < NI > > for u64x4_sse2 < YesS3 , YesS4 , NI > where u64x2_sse2 < YesS3 , YesS4 , NI > : RotateEachWord64 + RotateEachWord32 + BSwap , Avx2Machine < NI > : Machine , u64x4_sse2 < YesS3 , YesS4 , NI > : MultiLane < [u64 ; 4] > + Vec4 < u64 > + Words4 , { }
    };
}

impl_184!()