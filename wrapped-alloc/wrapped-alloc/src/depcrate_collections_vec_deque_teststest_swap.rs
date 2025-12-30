// Generated macro for test_swap (function)
macro_rules! Depcrate_collections_vec_deque_teststest_swap {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_swap"}
// Dependencies: {}
# [test] fn test_swap () { let mut tester = VecDeque :: new () ; tester . push_back (1) ; tester . push_back (2) ; tester . push_back (3) ; assert_eq ! (tester , [1 , 2 , 3]) ; tester . swap (0 , 0) ; assert_eq ! (tester , [1 , 2 , 3]) ; tester . swap (0 , 1) ; assert_eq ! (tester , [2 , 1 , 3]) ; tester . swap (2 , 1) ; assert_eq ! (tester , [2 , 3 , 1]) ; tester . swap (1 , 2) ; assert_eq ! (tester , [2 , 1 , 3]) ; tester . swap (0 , 2) ; assert_eq ! (tester , [3 , 1 , 2]) ; tester . swap (2 , 2) ; assert_eq ! (tester , [3 , 1 , 2]) ; }
};
}
