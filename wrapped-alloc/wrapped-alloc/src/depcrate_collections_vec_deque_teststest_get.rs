// Generated macro for test_get (function)
macro_rules! Depcrate_collections_vec_deque_teststest_get {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_get"}
// Dependencies: {}
# [test] fn test_get () { let mut tester = VecDeque :: new () ; tester . push_back (1) ; tester . push_back (2) ; tester . push_back (3) ; assert_eq ! (tester . len () , 3) ; assert_eq ! (tester . get (1) , Some (& 2)) ; assert_eq ! (tester . get (2) , Some (& 3)) ; assert_eq ! (tester . get (0) , Some (& 1)) ; assert_eq ! (tester . get (3) , None) ; tester . remove (0) ; assert_eq ! (tester . len () , 2) ; assert_eq ! (tester . get (0) , Some (& 2)) ; assert_eq ! (tester . get (1) , Some (& 3)) ; assert_eq ! (tester . get (2) , None) ; }
};
}
