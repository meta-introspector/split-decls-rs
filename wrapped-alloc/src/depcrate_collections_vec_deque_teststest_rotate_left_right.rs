// Generated macro for test_rotate_left_right (function)
macro_rules! Depcrate_collections_vec_deque_teststest_rotate_left_right {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_rotate_left_right"}
// Dependencies: {}
# [test] fn test_rotate_left_right () { let mut tester : VecDeque < _ > = (1 ..= 10) . collect () ; tester . reserve (1) ; assert_eq ! (tester . len () , 10) ; tester . rotate_left (0) ; assert_eq ! (tester , [1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10]) ; tester . rotate_right (0) ; assert_eq ! (tester , [1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10]) ; tester . rotate_left (3) ; assert_eq ! (tester , [4 , 5 , 6 , 7 , 8 , 9 , 10 , 1 , 2 , 3]) ; tester . rotate_right (5) ; assert_eq ! (tester , [9 , 10 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8]) ; tester . rotate_left (tester . len ()) ; assert_eq ! (tester , [9 , 10 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8]) ; tester . rotate_right (tester . len ()) ; assert_eq ! (tester , [9 , 10 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8]) ; tester . rotate_left (1) ; assert_eq ! (tester , [10 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9]) ; }
};
}
