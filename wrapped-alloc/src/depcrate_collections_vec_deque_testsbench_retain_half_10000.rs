// Generated macro for bench_retain_half_10000 (function)
macro_rules! Depcrate_collections_vec_deque_testsbench_retain_half_10000 {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"bench_retain_half_10000"}
// Dependencies: {}
# [bench] fn bench_retain_half_10000 (b : & mut test :: Bencher) { let size = if cfg ! (miri) { 1000 } else { 100000 } ; let v = (1 .. size) . collect :: < VecDeque < u32 > > () ; b . iter (| | { let mut v = v . clone () ; v . retain (| x | * x > size / 2) }) }
};
}
