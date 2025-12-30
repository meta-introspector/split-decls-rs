// Generated macro for test_get (function)
macro_rules! Depcrate_tests_dictionarytest_get {
() => {
// Module: crate::tests::dictionary
// Provides: {"test_get"}
// Dependencies: {}
# [test] fn test_get () { let dict = sample_dict ("abcd") ; assert ! (dict . objectForKey (ns_string ! ("abcd")) . is_some ()) ; assert ! (dict . objectForKey (ns_string ! ("abcde")) . is_none ()) ; }
};
}
