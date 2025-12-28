macro_rules! deps {
    () => {
        YesS3!();
        YesS4!();
        MultiLane!();
        RotateEachWord32!();
        Vec4!();
        BSwap!();
        Avx2Machine!();
        Machine!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < NI : Copy > u32x4 < Avx2Machine < NI > > for u32x4_sse2 < YesS3 , YesS4 , NI > where u32x4_sse2 < YesS3 , YesS4 , NI > : RotateEachWord32 + BSwap + MultiLane < [u32 ; 4] > + Vec4 < u32 > , Machine86 < YesS3 , YesS4 , NI > : Machine , { }
    };
}

impl_144!()