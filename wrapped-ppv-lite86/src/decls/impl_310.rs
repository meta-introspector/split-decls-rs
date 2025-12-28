macro_rules! deps {
    () => {
        Store!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        impl Store < vec128_storage > for u128x1_generic { # [inline (always)] unsafe fn unpack (s : vec128_storage) -> Self { Self ([o_of_q (s . q) ; 1]) } }
    };
}

impl_310!()