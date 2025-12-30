// Generated macro for new_different_lengths (function)
macro_rules! Depcrate_tests_dictionarynew_different_lengths {
() => {
// Module: crate::tests::dictionary
// Provides: {"new_different_lengths"}
// Dependencies: {}
# [test] # [should_panic = "key slice and object slice should have the same length"] fn new_different_lengths () { let dict = NSDictionary :: from_retained_objects (& [ns_string ! ("a") , ns_string ! ("b") , ns_string ! ("c")] , & [NSObject :: new () , NSObject :: new ()] ,) ; assert_eq ! (dict . len () , 2) ; }
};
}
