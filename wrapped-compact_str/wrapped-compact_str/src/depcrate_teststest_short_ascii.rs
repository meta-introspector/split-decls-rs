// Generated macro for test_short_ascii (function)
macro_rules! Depcrate_teststest_short_ascii {
() => {
// Module: crate::tests
// Provides: {"test_short_ascii"}
// Dependencies: {}
# [test] fn test_short_ascii () { let strs = vec ! ["nyc" , "statue" , "liberty" , "img_1234.png"] ; for s in strs { let compact = CompactString :: new (s) ; assert_eq ! (compact , s) ; assert_eq ! (s , compact) ; assert ! (! compact . is_heap_allocated ()) ; } }
};
}
