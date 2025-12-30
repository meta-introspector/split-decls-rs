// Generated macro for avx2_detected (function)
macro_rules! Depcrate_platformavx2_detected {
() => {
// Module: crate::platform
// Provides: {"avx2_detected"}
// Dependencies: {}
# [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] # [inline (always)] pub fn avx2_detected () -> bool { if cfg ! (miri) { return false ; } if cfg ! (feature = "no_avx2") { return false ; } cpufeatures :: new ! (has_avx2 , "avx2") ; has_avx2 :: get () }
};
}
