// Generated macro for impl_35 (impl)
macro_rules! Depcrate_softimpl_35 {
() => {
// Module: crate::soft
// Provides: {"impl_35"}
// Dependencies: {}
impl < W : BSwap + Copy , G > BSwap for x2 < W , G > { # [inline (always)] fn bswap (self) -> Self { x2 :: new ([self . 0 [0] . bswap () , self . 0 [1] . bswap ()]) } }
};
}
