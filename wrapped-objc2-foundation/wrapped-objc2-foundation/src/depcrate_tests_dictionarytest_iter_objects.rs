// Generated macro for test_iter_objects (function)
macro_rules! Depcrate_tests_dictionarytest_iter_objects {
() => {
// Module: crate::tests::dictionary
// Provides: {"test_iter_objects"}
// Dependencies: {}
# [test] # [cfg (feature = "NSEnumerator")] fn test_iter_objects () { let dict = sample_dict ("abcd") ; assert_eq ! (dict . objects () . count () , 1) ; }
};
}
