// Generated macro for test_arrays (function)
macro_rules! Depcrate_tests_dictionarytest_arrays {
() => {
// Module: crate::tests::dictionary
// Provides: {"test_arrays"}
// Dependencies: {}
# [test] # [cfg (feature = "NSArray")] fn test_arrays () { let dict = sample_dict ("abcd") ; let keys = dict . allKeys () ; assert_eq ! (keys . len () , 1) ; assert_eq ! (keys . objectAtIndex (0) . to_string () , "abcd") ; let objs = dict . allValues () ; assert_eq ! (objs . len () , 1) ; }
};
}
