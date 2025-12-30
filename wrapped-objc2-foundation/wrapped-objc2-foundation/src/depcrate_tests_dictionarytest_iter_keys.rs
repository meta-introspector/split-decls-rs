// Generated macro for test_iter_keys (function)
macro_rules! Depcrate_tests_dictionarytest_iter_keys {
() => {
// Module: crate::tests::dictionary
// Provides: {"test_iter_keys"}
// Dependencies: {}
# [test] # [cfg (feature = "NSEnumerator")] fn test_iter_keys () { let dict = sample_dict ("abcd") ; assert_eq ! (dict . keys () . count () , 1) ; assert_eq ! (dict . keys () . next () . unwrap () . to_string () , "abcd") ; }
};
}
