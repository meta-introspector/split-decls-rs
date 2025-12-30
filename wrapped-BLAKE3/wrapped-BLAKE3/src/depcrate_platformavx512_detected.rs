// Generated macro for avx512_detected (function)
macro_rules! Depcrate_platformavx512_detected {
() => {
// Module: crate::platform
// Provides: {"avx512_detected"}
// Dependencies: {}
# [cfg (blake3_avx512_ffi)] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] # [inline (always)] pub fn avx512_detected () -> bool { if cfg ! (miri) { return false ; } if cfg ! (feature = "no_avx512") { return false ; } cpufeatures :: new ! (has_avx512 , "avx512f" , "avx512vl") ; has_avx512 :: get () }
};
}
