// Generated macro for impl_64 (impl)
macro_rules! Depcrate_softimpl_64 {
() => {
// Module: crate::soft
// Provides: {"impl_64"}
// Dependencies: {}
impl < W : Copy + Store < vec128_storage > > Store < vec512_storage > for x4 < W > { # [inline (always)] unsafe fn unpack (p : vec512_storage) -> Self { let p = p . split128 () ; x4 ([W :: unpack (p [0]) , W :: unpack (p [1]) , W :: unpack (p [2]) , W :: unpack (p [3]) ,]) } }
};
}
