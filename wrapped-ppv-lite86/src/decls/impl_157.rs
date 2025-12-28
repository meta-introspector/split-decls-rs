macro_rules! deps {
    () => {
        YesS3!();
        BSwap!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < S4 , NI > BSwap for u32x4_sse2 < YesS3 , S4 , NI > { # [inline (always)] fn bswap (self) -> Self { Self :: new (unsafe { let k = _mm_set_epi64x (0x0c0d_0e0f_0809_0a0b , 0x0405_0607_0001_0203) ; _mm_shuffle_epi8 (self . x , k) }) } }
    };
}

impl_157!();