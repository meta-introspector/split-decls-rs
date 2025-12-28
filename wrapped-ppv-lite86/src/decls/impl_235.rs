macro_rules! impl_235 {
    () => {
        impl vec256_storage { # [inline (always)] pub fn new128 (xs : [vec128_storage ; 2]) -> Self { Self { sse2 : xs } } # [inline (always)] pub fn split128 (self) -> [vec128_storage ; 2] { unsafe { self . sse2 } } }
    };
}

impl_235!()