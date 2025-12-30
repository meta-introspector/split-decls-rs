// Generated macro for test_keys_and_objects (function)
macro_rules! Depcrate_tests_dictionarytest_keys_and_objects {
() => {
// Module: crate::tests::dictionary
// Provides: {"test_keys_and_objects"}
// Dependencies: {}
# [test] fn test_keys_and_objects () { let dict = sample_dict ("abcd") ; let (keys , objs) = dict . to_vecs () ; assert_eq ! (keys . len () , 1) ; assert_eq ! (objs . len () , 1) ; assert_eq ! (keys [0] . to_string () , "abcd") ; assert_eq ! (objs [0] , dict . objectForKey (& keys [0]) . unwrap ()) ; }
};
}
