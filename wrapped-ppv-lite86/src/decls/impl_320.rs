macro_rules! deps {
    () => {
        BSwap!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl BSwap for u32x4_generic { # [inline (always)] fn bswap (self) -> Self { dmap (self , | x | x . swap_bytes ()) } }
    };
}

impl_320!()