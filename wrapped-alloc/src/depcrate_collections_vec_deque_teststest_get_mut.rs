// Generated macro for test_get_mut (function)
macro_rules! Depcrate_collections_vec_deque_teststest_get_mut {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_get_mut"}
// Dependencies: {}
# [test] fn test_get_mut () { let mut tester = VecDeque :: new () ; tester . push_back (1) ; tester . push_back (2) ; tester . push_back (3) ; assert_eq ! (tester . len () , 3) ; if let Some (elem) = tester . get_mut (0) { assert_eq ! (* elem , 1) ; * elem = 10 ; } if let Some (elem) = tester . get_mut (2) { assert_eq ! (* elem , 3) ; * elem = 30 ; } assert_eq ! (tester . get (0) , Some (& 10)) ; assert_eq ! (tester . get (2) , Some (& 30)) ; assert_eq ! (tester . get_mut (3) , None) ; tester . remove (2) ; assert_eq ! (tester . len () , 2) ; assert_eq ! (tester . get (0) , Some (& 10)) ; assert_eq ! (tester . get (1) , Some (& 2)) ; assert_eq ! (tester . get (2) , None) ; }
};
}
