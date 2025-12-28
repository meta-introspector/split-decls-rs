macro_rules! deps {
    () => {
        Store!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl Store < vec128_storage > for vec128_storage { # [inline (always)] unsafe fn unpack (p : vec128_storage) -> Self { p } }
    };
}

impl_226!();