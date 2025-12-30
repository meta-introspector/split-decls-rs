// Generated macro for bench_push_front_100 (function)
macro_rules! Depcrate_collections_vec_deque_testsbench_push_front_100 {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"bench_push_front_100"}
// Dependencies: {}
# [bench] fn bench_push_front_100 (b : & mut test :: Bencher) { let mut deq = VecDeque :: with_capacity (101) ; b . iter (| | { for i in 0 .. 100 { deq . push_front (i) ; } deq . head = 0 ; deq . len = 0 ; }) }
};
}
