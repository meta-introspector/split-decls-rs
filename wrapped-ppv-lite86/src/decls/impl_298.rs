macro_rules! deps {
    () => {
        RotateEachWord32!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl RotateEachWord32 for u128x1_generic { # [inline (always)] fn rotate_each_word_right7 (self) -> Self { Self ([rotate_u128_right (self . 0 [0] , 7)]) } # [inline (always)] fn rotate_each_word_right8 (self) -> Self { Self ([rotate_u128_right (self . 0 [0] , 8)]) } # [inline (always)] fn rotate_each_word_right11 (self) -> Self { Self ([rotate_u128_right (self . 0 [0] , 11)]) } # [inline (always)] fn rotate_each_word_right12 (self) -> Self { Self ([rotate_u128_right (self . 0 [0] , 12)]) } # [inline (always)] fn rotate_each_word_right16 (self) -> Self { Self ([rotate_u128_right (self . 0 [0] , 16)]) } # [inline (always)] fn rotate_each_word_right20 (self) -> Self { Self ([rotate_u128_right (self . 0 [0] , 20)]) } # [inline (always)] fn rotate_each_word_right24 (self) -> Self { Self ([rotate_u128_right (self . 0 [0] , 24)]) } # [inline (always)] fn rotate_each_word_right25 (self) -> Self { Self ([rotate_u128_right (self . 0 [0] , 25)]) } }
    };
}

impl_298!();