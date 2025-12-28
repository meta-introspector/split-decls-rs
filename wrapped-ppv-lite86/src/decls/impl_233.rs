macro_rules! impl_233 {
    () => {
        impl From < [u64 ; 4] > for vec256_storage { # [inline (always)] fn from (u64x4 : [u64 ; 4]) -> Self { vec256_storage { u64x4 } } }
    };
}

impl_233!()