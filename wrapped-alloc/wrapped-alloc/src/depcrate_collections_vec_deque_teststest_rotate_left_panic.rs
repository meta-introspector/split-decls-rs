// Generated macro for test_rotate_left_panic (function)
macro_rules! Depcrate_collections_vec_deque_teststest_rotate_left_panic {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_rotate_left_panic"}
// Dependencies: {}
# [test] # [should_panic = "assertion failed: n <= self.len()"] fn test_rotate_left_panic () { let mut tester : VecDeque < _ > = (1 ..= 10) . collect () ; tester . rotate_left (tester . len () + 1) ; }
};
}
