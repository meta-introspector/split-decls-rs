// Generated macro for test_reserve_exact (function)
macro_rules! Depcrate_collections_vec_deque_teststest_reserve_exact {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_reserve_exact"}
// Dependencies: {}
# [test] fn test_reserve_exact () { let mut tester : VecDeque < i32 > = VecDeque :: with_capacity (1) ; assert_eq ! (tester . capacity () , 1) ; tester . reserve_exact (50) ; assert_eq ! (tester . capacity () , 50) ; tester . reserve_exact (40) ; assert_eq ! (tester . capacity () , 50) ; tester . reserve_exact (200) ; assert_eq ! (tester . capacity () , 200) ; }
};
}
