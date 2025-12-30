// Generated macro for test_iter_mutation_detection (function)
macro_rules! Depcrate_tests_mutable_arraytest_iter_mutation_detection {
() => {
// Module: crate::tests::mutable_array
// Provides: {"test_iter_mutation_detection"}
// Dependencies: {}
# [test] # [should_panic = "mutation detected during enumeration"] # [cfg_attr (feature = "gnustep-1-7" , ignore = "thread safety issues regarding initialization")] fn test_iter_mutation_detection () { let array = NSMutableArray :: from_retained_slice (& [NSObject :: new () , NSObject :: new ()]) ; for item in & array { array . removeObject (& item) ; } }
};
}
