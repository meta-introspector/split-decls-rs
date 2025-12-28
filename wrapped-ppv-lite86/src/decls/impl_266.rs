macro_rules! impl_266 {
    () => {
        impl vec256_storage { # [inline (always)] pub fn new128 (v128 : [vec128_storage ; 2]) -> Self { Self { v128 } } # [inline (always)] pub fn split128 (self) -> [vec128_storage ; 2] { self . v128 } }
    };
}

impl_266!()