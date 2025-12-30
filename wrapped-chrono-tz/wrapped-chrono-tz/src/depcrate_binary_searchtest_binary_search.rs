// Generated macro for test_binary_search (function)
macro_rules! Depcrate_binary_searchtest_binary_search {
() => {
// Module: crate::binary_search
// Provides: {"test_binary_search"}
// Dependencies: {}
# [test] fn test_binary_search () { assert_eq ! (binary_search (0 , 5000 , | x | x . cmp (& 1337)) , Ok (1337)) ; assert_eq ! (binary_search (0 , 5000 , | x | x . cmp (& 9000)) , Err (5000)) ; assert_eq ! (binary_search (30 , 50 , | x | x . cmp (& 42)) , Ok (42)) ; assert_eq ! (binary_search (300 , 500 , | x | x . cmp (& 42)) , Err (300)) ; assert_eq ! (binary_search (0 , 500 , | x | if x < 42 { Ordering :: Less } else { Ordering :: Greater }) , Err (42)) ; }
};
}
