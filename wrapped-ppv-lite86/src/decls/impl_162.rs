macro_rules! deps {
    () => {
        YesS3!();
        BSwap!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl < S4 , NI > BSwap for u128x1_sse2 < YesS3 , S4 , NI > { # [inline (always)] fn bswap (self) -> Self { Self :: new (unsafe { let k = _mm_set_epi64x (0x0f0e_0d0c_0b0a_0908 , 0x0706_0504_0302_0100) ; _mm_shuffle_epi8 (self . x , k) }) } }
    };
}

impl_162!()