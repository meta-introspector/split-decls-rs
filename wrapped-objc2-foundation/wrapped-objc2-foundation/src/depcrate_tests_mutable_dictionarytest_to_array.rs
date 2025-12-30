// Generated macro for test_to_array (function)
macro_rules! Depcrate_tests_mutable_dictionarytest_to_array {
() => {
// Module: crate::tests::mutable_dictionary
// Provides: {"test_to_array"}
// Dependencies: {}
# [test] # [cfg (feature = "NSArray")] fn test_to_array () { let dict = sample_dict () ; let array = dict . allValues () ; assert_eq ! (array . len () , 3) ; }
};
}
