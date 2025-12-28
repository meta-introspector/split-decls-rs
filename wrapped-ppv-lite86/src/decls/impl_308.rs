macro_rules! deps {
    () => {
        Store!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl Store < vec128_storage > for u32x4_generic { # [inline (always)] unsafe fn unpack (s : vec128_storage) -> Self { Self (s . d) } }
    };
}

impl_308!();