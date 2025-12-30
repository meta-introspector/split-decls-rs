// Generated macro for impl_68 (impl)
macro_rules! Depcrate_softimpl_68 {
() => {
// Module: crate::soft
// Provides: {"impl_68"}
// Dependencies: {}
impl < W : BSwap + Copy > BSwap for x4 < W > { # [inline (always)] fn bswap (self) -> Self { x4 ([self . 0 [0] . bswap () , self . 0 [1] . bswap () , self . 0 [2] . bswap () , self . 0 [3] . bswap () ,]) } }
};
}
