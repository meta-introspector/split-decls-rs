macro_rules! impl_270 {
    () => {
        impl vec512_storage { # [inline (always)] pub fn new128 (v128 : [vec128_storage ; 4]) -> Self { Self { v128 } } # [inline (always)] pub fn split128 (self) -> [vec128_storage ; 4] { self . v128 } }
    };
}

impl_270!();