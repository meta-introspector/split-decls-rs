macro_rules! deps {
    () => {
        NoS3!();
        BSwap!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl < S4 , NI > BSwap for u128x1_sse2 < NoS3 , S4 , NI > { # [inline (always)] fn bswap (self) -> Self { unimplemented ! () } }
    };
}

impl_163!()