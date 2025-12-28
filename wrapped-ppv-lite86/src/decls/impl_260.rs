macro_rules! impl_260 {
    () => {
        impl From < [u64 ; 2] > for vec128_storage { # [inline (always)] fn from (q : [u64 ; 2]) -> Self { Self { q } } }
    };
}

impl_260!()