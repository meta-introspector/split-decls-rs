macro_rules! deps {
    () => {
        BSwap!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < W : BSwap + Copy , G > BSwap for x2 < W , G > { # [inline (always)] fn bswap (self) -> Self { x2 :: new ([self . 0 [0] . bswap () , self . 0 [1] . bswap ()]) } }
    };
}

impl_29!()