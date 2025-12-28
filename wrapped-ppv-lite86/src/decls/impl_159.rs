macro_rules! deps {
    () => {
        NoS3!();
        BSwap!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl < S4 , NI > BSwap for u32x4_sse2 < NoS3 , S4 , NI > { # [inline (always)] fn bswap (self) -> Self { Self :: new (bswap32_s2 (self . x)) } }
    };
}

impl_159!();