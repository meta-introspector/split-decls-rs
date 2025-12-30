// Generated macro for bench (module)
macro_rules! Depcrate_dynimagebench {
() => {
// Module: crate::dynimage
// Provides: {"bench"}
// Dependencies: {}
# [cfg (test)] mod bench { # [bench] # [cfg (feature = "benchmarks")] fn bench_conversion (b : & mut test :: Bencher) { let a = super :: DynamicImage :: ImageRgb8 (crate :: ImageBuffer :: new (1000 , 1000)) ; b . iter (| | a . to_luma8 ()) ; b . bytes = 1000 * 1000 * 3 } }
};
}
