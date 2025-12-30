// Generated macro for test_swap_panic (function)
macro_rules! Depcrate_collections_vec_deque_teststest_swap_panic {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_swap_panic"}
// Dependencies: {}
# [test] # [should_panic = "assertion failed: j < self.len()"] fn test_swap_panic () { let mut tester = VecDeque :: new () ; tester . push_back (1) ; tester . push_back (2) ; tester . push_back (3) ; tester . swap (2 , 3) ; }
};
}
