// Generated macro for impl_258 (impl)
macro_rules! Depcrate_x86_64impl_258 {
() => {
// Module: crate::x86_64
// Provides: {"impl_258"}
// Dependencies: {}
impl vec256_storage { # [inline (always)] pub fn new128 (xs : [vec128_storage ; 2]) -> Self { Self { sse2 : xs } } # [inline (always)] pub fn split128 (self) -> [vec128_storage ; 2] { unsafe { self . sse2 } } }
};
}
