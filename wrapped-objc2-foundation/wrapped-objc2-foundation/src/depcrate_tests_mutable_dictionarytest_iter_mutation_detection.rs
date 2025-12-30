// Generated macro for test_iter_mutation_detection (function)
macro_rules! Depcrate_tests_mutable_dictionarytest_iter_mutation_detection {
() => {
// Module: crate::tests::mutable_dictionary
// Provides: {"test_iter_mutation_detection"}
// Dependencies: {}
# [test] # [should_panic = "mutation detected during enumeration"] fn test_iter_mutation_detection () { let dict = sample_dict () ; let mut iter = dict . keys () ; let _ = iter . next () ; dict . insert (& * NSNumber :: new_usize (1) , & NSObject :: new ()) ; let _ = iter . next () ; }
};
}
