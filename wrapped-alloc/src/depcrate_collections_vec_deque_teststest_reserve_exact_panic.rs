// Generated macro for test_reserve_exact_panic (function)
macro_rules! Depcrate_collections_vec_deque_teststest_reserve_exact_panic {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_reserve_exact_panic"}
// Dependencies: {}
# [test] # [should_panic = "capacity overflow"] fn test_reserve_exact_panic () { let mut tester : VecDeque < i32 > = VecDeque :: new () ; tester . reserve_exact (usize :: MAX) ; }
};
}
