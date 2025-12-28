macro_rules! deps {
    () => {
        YesS3!();
        BSwap!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < S4 , NI > BSwap for u64x2_sse2 < YesS3 , S4 , NI > { # [inline (always)] fn bswap (self) -> Self { Self :: new (unsafe { let k = _mm_set_epi64x (0x0809_0a0b_0c0d_0e0f , 0x0001_0203_0405_0607) ; _mm_shuffle_epi8 (self . x , k) }) } }
    };
}

impl_160!();