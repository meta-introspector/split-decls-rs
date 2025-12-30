// Generated macro for test_try_reserve (function)
macro_rules! Depcrate_collections_vec_deque_teststest_try_reserve {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_try_reserve"}
// Dependencies: {}
# [test] fn test_try_reserve () { let mut tester : VecDeque < i32 > = VecDeque :: with_capacity (1) ; assert ! (tester . capacity () == 1) ; assert_eq ! (tester . try_reserve (100) , Ok (())) ; assert ! (tester . capacity () >= 100) ; assert_eq ! (tester . try_reserve (50) , Ok (())) ; assert ! (tester . capacity () >= 100) ; assert_eq ! (tester . try_reserve (200) , Ok (())) ; assert ! (tester . capacity () >= 200) ; assert_eq ! (tester . try_reserve (0) , Ok (())) ; assert ! (tester . capacity () >= 200) ; assert ! (tester . try_reserve (usize :: MAX) . is_err ()) ; }
};
}
