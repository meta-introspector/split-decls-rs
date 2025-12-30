// Generated macro for test_binary_search_by (function)
macro_rules! Depcrate_collections_vec_deque_teststest_binary_search_by {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_binary_search_by"}
// Dependencies: {}
# [test] fn test_binary_search_by () { let tester : VecDeque < _ > = [0 , 1 , 1 , 2 , 3 , 5 , 8 , 13 , 21 , 34 , 55] . into () ; assert_eq ! (tester . binary_search_by (| x | x . cmp (& 0)) , Ok (0)) ; assert_eq ! (tester . binary_search_by (| x | x . cmp (& 5)) , Ok (5)) ; assert_eq ! (tester . binary_search_by (| x | x . cmp (& 55)) , Ok (10)) ; assert_eq ! (tester . binary_search_by (| x | x . cmp (& 4)) , Err (5)) ; assert_eq ! (tester . binary_search_by (| x | x . cmp (&- 1)) , Err (0)) ; assert ! (matches ! (tester . binary_search_by (| x | x . cmp (& 1)) , Ok (1 ..= 2))) ; }
};
}
