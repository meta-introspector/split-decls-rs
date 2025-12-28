macro_rules! deps {
    () => {
        Words4!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl < S3 , S4 , NI > Words4 for u32x4_sse2 < S3 , S4 , NI > { # [inline (always)] fn shuffle2301 (self) -> Self { Self :: new (unsafe { _mm_shuffle_epi32 (self . x , 0b0100_1110) }) } # [inline (always)] fn shuffle1230 (self) -> Self { Self :: new (unsafe { _mm_shuffle_epi32 (self . x , 0b1001_0011) }) } # [inline (always)] fn shuffle3012 (self) -> Self { Self :: new (unsafe { _mm_shuffle_epi32 (self . x , 0b0011_1001) }) } }
    };
}

impl_151!();