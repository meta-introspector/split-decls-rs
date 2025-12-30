// Generated macro for test_extend_from_empty_strs (function)
macro_rules! Depcrate_teststest_extend_from_empty_strs {
() => {
// Module: crate::tests
// Provides: {"test_extend_from_empty_strs"}
// Dependencies: {}
# [test] fn test_extend_from_empty_strs () { let strs = vec ! ["" , "" , "" , "" , "" , "" , "" , "" , "" , "" , "" , "" , "" , "" , "" , "" , "" , "" , "" , "" , "" , "" , "" , "" , "" ,] ; let compact : CompactString = strs . clone () . into_iter () . collect () ; assert_eq ! (compact , "") ; assert ! (compact . is_empty ()) ; assert ! (! compact . is_heap_allocated ()) ; }
};
}
