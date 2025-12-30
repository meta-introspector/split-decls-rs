// Generated macro for test_medium_ascii (function)
macro_rules! Depcrate_teststest_medium_ascii {
() => {
// Module: crate::tests
// Provides: {"test_medium_ascii"}
// Dependencies: {}
# [test] fn test_medium_ascii () { let strs = vec ! ["rustconf 2021" , "new york city" , "nyc pizza is good" , "test the 24 char limit!!" ,] ; for s in strs { let compact = CompactString :: new (s) ; assert_eq ! (compact , s) ; assert_eq ! (s , compact) ; # [cfg (target_pointer_width = "64")] let is_heap = false ; # [cfg (target_pointer_width = "32")] let is_heap = true ; assert_eq ! (compact . is_heap_allocated () , is_heap) ; } }
};
}
