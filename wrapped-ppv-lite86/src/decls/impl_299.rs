macro_rules! deps {
    () => {
        RotateEachWord64!();
    };
}

macro_rules! impl_299 {
    () => {
        deps!();
        impl RotateEachWord64 for u128x1_generic { # [inline (always)] fn rotate_each_word_right32 (self) -> Self { Self ([rotate_u128_right (self . 0 [0] , 32)]) } }
    };
}

impl_299!();