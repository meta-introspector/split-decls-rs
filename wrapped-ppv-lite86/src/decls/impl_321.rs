macro_rules! deps {
    () => {
        BSwap!();
    };
}

macro_rules! impl_321 {
    () => {
        deps!();
        impl BSwap for u64x2_generic { # [inline (always)] fn bswap (self) -> Self { qmap (self , | x | x . swap_bytes ()) } }
    };
}

impl_321!()