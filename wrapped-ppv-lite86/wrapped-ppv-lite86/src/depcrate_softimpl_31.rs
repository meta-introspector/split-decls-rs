// Generated macro for impl_31 (impl)
macro_rules! Depcrate_softimpl_31 {
() => {
// Module: crate::soft
// Provides: {"impl_31"}
// Dependencies: {}
impl < W : Copy + Store < vec128_storage > , G > Store < vec256_storage > for x2 < W , G > { # [inline (always)] unsafe fn unpack (p : vec256_storage) -> Self { let p = p . split128 () ; x2 :: new ([W :: unpack (p [0]) , W :: unpack (p [1])]) } }
};
}
