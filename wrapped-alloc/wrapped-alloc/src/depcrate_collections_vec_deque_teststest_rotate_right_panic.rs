// Generated macro for test_rotate_right_panic (function)
macro_rules! Depcrate_collections_vec_deque_teststest_rotate_right_panic {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_rotate_right_panic"}
// Dependencies: {}
# [test] # [should_panic = "assertion failed: n <= self.len()"] fn test_rotate_right_panic () { let mut tester : VecDeque < _ > = (1 ..= 10) . collect () ; tester . rotate_right (tester . len () + 1) ; }
};
}
