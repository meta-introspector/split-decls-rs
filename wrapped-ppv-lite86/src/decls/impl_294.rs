macro_rules! deps {
    () => {
        RotateEachWord32!();
    };
}

macro_rules! impl_294 {
    () => {
        deps!();
        impl RotateEachWord32 for u64x2_generic { # [inline (always)] fn rotate_each_word_right7 (self) -> Self { qmap (self , | x | x . rotate_right (7)) } # [inline (always)] fn rotate_each_word_right8 (self) -> Self { qmap (self , | x | x . rotate_right (8)) } # [inline (always)] fn rotate_each_word_right11 (self) -> Self { qmap (self , | x | x . rotate_right (11)) } # [inline (always)] fn rotate_each_word_right12 (self) -> Self { qmap (self , | x | x . rotate_right (12)) } # [inline (always)] fn rotate_each_word_right16 (self) -> Self { qmap (self , | x | x . rotate_right (16)) } # [inline (always)] fn rotate_each_word_right20 (self) -> Self { qmap (self , | x | x . rotate_right (20)) } # [inline (always)] fn rotate_each_word_right24 (self) -> Self { qmap (self , | x | x . rotate_right (24)) } # [inline (always)] fn rotate_each_word_right25 (self) -> Self { qmap (self , | x | x . rotate_right (25)) } }
    };
}

impl_294!()