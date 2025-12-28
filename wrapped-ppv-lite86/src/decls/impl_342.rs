macro_rules! deps {
    () => {
        Words4!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl Words4 for u32x4_generic { # [inline (always)] fn shuffle2301 (self) -> Self { self . swap64 () } # [inline (always)] fn shuffle1230 (self) -> Self { let x = self . 0 ; Self ([x [3] , x [0] , x [1] , x [2]]) } # [inline (always)] fn shuffle3012 (self) -> Self { let x = self . 0 ; Self ([x [1] , x [2] , x [3] , x [0]]) } }
    };
}

impl_342!()