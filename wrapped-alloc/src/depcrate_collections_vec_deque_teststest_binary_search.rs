// Generated macro for test_binary_search (function)
macro_rules! Depcrate_collections_vec_deque_teststest_binary_search {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_binary_search"}
// Dependencies: {}
# [test] fn test_binary_search () { let tester : VecDeque < _ > = [0 , 1 , 1 , 2 , 3 , 5 , 8 , 13 , 21 , 34 , 55] . into () ; assert_eq ! (tester . binary_search (& 0) , Ok (0)) ; assert_eq ! (tester . binary_search (& 5) , Ok (5)) ; assert_eq ! (tester . binary_search (& 55) , Ok (10)) ; assert_eq ! (tester . binary_search (& 4) , Err (5)) ; assert_eq ! (tester . binary_search (&- 1) , Err (0)) ; assert ! (matches ! (tester . binary_search (& 1) , Ok (1 ..= 2))) ; let tester : VecDeque < _ > = [1 , 2 , 2 , 2 , 2 , 3 , 3 , 3 , 3 , 3 , 3 , 3 , 3 , 3] . into () ; assert_eq ! (tester . binary_search (& 1) , Ok (0)) ; assert ! (matches ! (tester . binary_search (& 2) , Ok (1 ..= 4))) ; assert ! (matches ! (tester . binary_search (& 3) , Ok (5 ..= 13))) ; assert_eq ! (tester . binary_search (&- 2) , Err (0)) ; assert_eq ! (tester . binary_search (& 0) , Err (0)) ; assert_eq ! (tester . binary_search (& 4) , Err (14)) ; assert_eq ! (tester . binary_search (& 5) , Err (14)) ; }
};
}
