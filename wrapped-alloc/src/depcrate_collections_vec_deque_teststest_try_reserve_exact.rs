// Generated macro for test_try_reserve_exact (function)
macro_rules! Depcrate_collections_vec_deque_teststest_try_reserve_exact {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_try_reserve_exact"}
// Dependencies: {}
# [test] fn test_try_reserve_exact () { let mut tester : VecDeque < i32 > = VecDeque :: with_capacity (1) ; assert ! (tester . capacity () == 1) ; assert_eq ! (tester . try_reserve_exact (100) , Ok (())) ; assert ! (tester . capacity () >= 100) ; assert_eq ! (tester . try_reserve_exact (50) , Ok (())) ; assert ! (tester . capacity () >= 100) ; assert_eq ! (tester . try_reserve_exact (200) , Ok (())) ; assert ! (tester . capacity () >= 200) ; assert_eq ! (tester . try_reserve_exact (0) , Ok (())) ; assert ! (tester . capacity () >= 200) ; assert ! (tester . try_reserve_exact (usize :: MAX) . is_err ()) ; }
};
}
