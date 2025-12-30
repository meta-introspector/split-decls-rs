// Generated macro for test_remove (function)
macro_rules! Depcrate_tests_mutable_dictionarytest_remove {
() => {
// Module: crate::tests::mutable_dictionary
// Provides: {"test_remove"}
// Dependencies: {}
# [test] fn test_remove () { let dict = sample_dict () ; assert_eq ! (dict . len () , 3) ; dict . removeObjectForKey (& NSNumber :: new_i32 (1)) ; dict . removeObjectForKey (& NSNumber :: new_i32 (2)) ; dict . removeObjectForKey (& NSNumber :: new_i32 (1)) ; dict . removeObjectForKey (& NSNumber :: new_i32 (4)) ; assert_eq ! (dict . len () , 1) ; }
};
}
