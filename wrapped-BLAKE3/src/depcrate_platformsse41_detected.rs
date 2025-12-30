// Generated macro for sse41_detected (function)
macro_rules! Depcrate_platformsse41_detected {
() => {
// Module: crate::platform
// Provides: {"sse41_detected"}
// Dependencies: {}
# [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] # [inline (always)] pub fn sse41_detected () -> bool { if cfg ! (miri) { return false ; } if cfg ! (feature = "no_sse41") { return false ; } cpufeatures :: new ! (has_sse41 , "sse4.1") ; has_sse41 :: get () }
};
}
