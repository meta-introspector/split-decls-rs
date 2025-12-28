macro_rules! deps {
    () => {
        MultiLane!();
        Vec4!();
        BSwap!();
        Machine!();
        Words4!();
        RotateEachWord64!();
        RotateEachWord32!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < S3 : Copy , S4 : Copy , NI : Copy > u64x4 < Machine86 < S3 , S4 , NI > > for u64x4_sse2 < S3 , S4 , NI > where u64x2_sse2 < S3 , S4 , NI > : RotateEachWord64 + RotateEachWord32 + BSwap , Machine86 < S3 , S4 , NI > : Machine , u64x4_sse2 < S3 , S4 , NI > : MultiLane < [u64 ; 4] > + Vec4 < u64 > + Words4 , { }
    };
}

impl_180!();