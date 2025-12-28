macro_rules! impl_305 {
    () => {
        impl From < u32x4_generic > for vec128_storage { # [inline (always)] fn from (d : u32x4_generic) -> Self { Self { d : d . 0 } } }
    };
}

impl_305!()