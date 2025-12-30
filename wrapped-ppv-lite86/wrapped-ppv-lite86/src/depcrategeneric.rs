// Generated macro for generic (module)
macro_rules! Depcrategeneric {
() => {
// Module: crate
// Provides: {"generic"}
// Dependencies: {}
# [cfg (any (feature = "no_simd" , miri , not (target_arch = "x86_64") , all (target_arch = "x86_64" , not (target_feature = "sse2"))))] pub mod generic ;
};
}
