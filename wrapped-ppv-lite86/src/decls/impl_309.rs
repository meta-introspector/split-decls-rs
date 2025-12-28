macro_rules! deps {
    () => {
        Store!();
    };
}

macro_rules! impl_309 {
    () => {
        deps!();
        impl Store < vec128_storage > for u64x2_generic { # [inline (always)] unsafe fn unpack (s : vec128_storage) -> Self { Self (s . q) } }
    };
}

impl_309!()