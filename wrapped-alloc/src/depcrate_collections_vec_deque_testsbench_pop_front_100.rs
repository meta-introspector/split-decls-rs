// Generated macro for bench_pop_front_100 (function)
macro_rules! Depcrate_collections_vec_deque_testsbench_pop_front_100 {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"bench_pop_front_100"}
// Dependencies: {}
# [bench] fn bench_pop_front_100 (b : & mut test :: Bencher) { let size = 100 ; let mut deq = VecDeque :: < i32 > :: with_capacity (size + 1) ; unsafe { deq . ptr () . write_bytes (0u8 , size + 1) } ; b . iter (| | { deq . head = 0 ; deq . len = 100 ; while ! deq . is_empty () { test :: black_box (deq . pop_front ()) ; } }) }
};
}
