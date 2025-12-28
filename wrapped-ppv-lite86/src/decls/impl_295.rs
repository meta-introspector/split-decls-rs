macro_rules! deps {
    () => {
        RotateEachWord64!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        impl RotateEachWord64 for u64x2_generic { # [inline (always)] fn rotate_each_word_right32 (self) -> Self { qmap (self , | x | x . rotate_right (32)) } }
    };
}

impl_295!()