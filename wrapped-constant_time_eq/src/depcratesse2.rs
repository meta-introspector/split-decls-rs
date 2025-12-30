// Generated macro for sse2 (module)
macro_rules! Depcratesse2 {
() => {
// Module: crate
// Provides: {"sse2"}
// Dependencies: {}
# [cfg (all (any (target_arch = "x86" , target_arch = "x86_64") , target_feature = "sse2" , not (miri)))] mod sse2 ;
};
}
