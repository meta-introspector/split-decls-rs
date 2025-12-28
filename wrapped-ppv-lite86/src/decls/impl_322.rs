macro_rules! deps {
    () => {
        BSwap!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        impl BSwap for u128x1_generic { # [inline (always)] fn bswap (self) -> Self { omap (self , | x | x . swap_bytes ()) } }
    };
}

impl_322!();