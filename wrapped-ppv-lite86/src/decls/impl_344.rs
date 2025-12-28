macro_rules! deps {
    () => {
        Words4!();
    };
}

macro_rules! impl_344 {
    () => {
        deps!();
        impl Words4 for u64x4_generic { # [inline (always)] fn shuffle2301 (self) -> Self { x2 :: new ([self . 0 [1] , self . 0 [0]]) } # [inline (always)] fn shuffle1230 (self) -> Self { unimplemented ! () } # [inline (always)] fn shuffle3012 (self) -> Self { unimplemented ! () } }
    };
}

impl_344!();