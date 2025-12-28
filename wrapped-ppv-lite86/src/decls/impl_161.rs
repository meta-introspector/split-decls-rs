macro_rules! deps {
    () => {
        BSwap!();
        NoS3!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl < S4 , NI > BSwap for u64x2_sse2 < NoS3 , S4 , NI > { # [inline (always)] fn bswap (self) -> Self { Self :: new (unsafe { bswap32_s2 (_mm_shuffle_epi32 (self . x , 0b1011_0001)) }) } }
    };
}

impl_161!();