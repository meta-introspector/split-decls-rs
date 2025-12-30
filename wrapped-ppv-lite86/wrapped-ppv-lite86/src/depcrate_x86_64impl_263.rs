// Generated macro for impl_263 (impl)
macro_rules! Depcrate_x86_64impl_263 {
() => {
// Module: crate::x86_64
// Provides: {"impl_263"}
// Dependencies: {}
impl vec512_storage { # [inline (always)] pub fn new128 (xs : [vec128_storage ; 4]) -> Self { Self { sse2 : xs } } # [inline (always)] pub fn split128 (self) -> [vec128_storage ; 4] { unsafe { self . sse2 } } }
};
}
