// Generated macro for test_get (function)
macro_rules! Depcrate_tests_mutable_dictionarytest_get {
() => {
// Module: crate::tests::mutable_dictionary
// Provides: {"test_get"}
// Dependencies: {}
# [test] fn test_get () { let dict = sample_dict_mut () ; assert ! (dict . objectForKey (& NSNumber :: new_i32 (1)) . is_some ()) ; assert ! (dict . objectForKey (& NSNumber :: new_i32 (2)) . is_some ()) ; assert ! (dict . objectForKey (& NSNumber :: new_i32 (4)) . is_none ()) ; }
};
}
