macro_rules! deps {
    () => {
        RotateEachWord64!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < S3 : Copy , S4 : Copy , NI : Copy > RotateEachWord64 for u64x2_sse2 < S3 , S4 , NI > { # [inline (always)] fn rotate_each_word_right32 (self) -> Self { Self :: new (unsafe { _mm_shuffle_epi32 (self . x , 0b10110001) }) } }
    };
}

impl_115!()