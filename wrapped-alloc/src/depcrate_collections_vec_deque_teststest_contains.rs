// Generated macro for test_contains (function)
macro_rules! Depcrate_collections_vec_deque_teststest_contains {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_contains"}
// Dependencies: {}
# [test] fn test_contains () { let mut tester = VecDeque :: new () ; tester . push_back (1) ; tester . push_back (2) ; tester . push_back (3) ; assert ! (tester . contains (& 1)) ; assert ! (tester . contains (& 3)) ; assert ! (! tester . contains (& 0)) ; assert ! (! tester . contains (& 4)) ; tester . remove (0) ; assert ! (! tester . contains (& 1)) ; assert ! (tester . contains (& 2)) ; assert ! (tester . contains (& 3)) ; }
};
}
