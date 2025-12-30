// Generated macro for impl_43 (impl)
macro_rules! Depcrate_rngimpl_43 {
() => {
// Module: crate::rng
// Provides: {"impl_43"}
// Dependencies: {}
impl From < [u8 ; 32] > for Seed { # [cfg (feature = "zeroize")] fn from (mut value : [u8 ; 32]) -> Self { let input = Self (value) ; value . zeroize () ; input } # [cfg (not (feature = "zeroize"))] fn from (value : [u8 ; 32]) -> Self { Self (value) } }
};
}
