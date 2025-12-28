macro_rules! impl_228 {
    () => {
        impl From < [u32 ; 4] > for vec128_storage { # [inline (always)] fn from (u32x4 : [u32 ; 4]) -> Self { vec128_storage { u32x4 } } }
    };
}

impl_228!()