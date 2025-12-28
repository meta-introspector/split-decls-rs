macro_rules! deps {
    () => {
        BSwap!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < W : BSwap + Copy > BSwap for x4 < W > { # [inline (always)] fn bswap (self) -> Self { x4 ([self . 0 [0] . bswap () , self . 0 [1] . bswap () , self . 0 [2] . bswap () , self . 0 [3] . bswap () ,]) } }
    };
}

impl_62!();