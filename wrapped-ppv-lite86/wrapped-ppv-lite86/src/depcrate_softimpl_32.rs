// Generated macro for impl_32 (impl)
macro_rules! Depcrate_softimpl_32 {
() => {
// Module: crate::soft
// Provides: {"impl_32"}
// Dependencies: {}
impl < W , G > From < x2 < W , G > > for vec256_storage where W : Copy , vec128_storage : From < W > , { # [inline (always)] fn from (x : x2 < W , G >) -> Self { vec256_storage :: new128 ([x . 0 [0] . into () , x . 0 [1] . into ()]) } }
};
}
