macro_rules! impl_240 {
    () => {
        impl vec512_storage { # [inline (always)] pub fn new128 (xs : [vec128_storage ; 4]) -> Self { Self { sse2 : xs } } # [inline (always)] pub fn split128 (self) -> [vec128_storage ; 4] { unsafe { self . sse2 } } }
    };
}

impl_240!();