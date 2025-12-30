// Generated macro for impl_257 (impl)
macro_rules! Depcrate_non_zeroimpl_257 {
() => {
// Module: crate::non_zero
// Provides: {"impl_257"}
// Dependencies: {}
# [cfg (feature = "zeroize")] impl < T : zeroize :: Zeroize + Zero > zeroize :: Zeroize for NonZero < T > { fn zeroize (& mut self) { self . 0 . zeroize () ; } }
};
}
