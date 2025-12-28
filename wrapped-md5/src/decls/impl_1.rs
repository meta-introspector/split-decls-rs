macro_rules! deps {
    () => {
        Digest!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl core :: convert :: From < Digest > for [u8 ; 16] { # [inline] fn from (digest : Digest) -> Self { digest . 0 } }
    };
}

impl_1!();