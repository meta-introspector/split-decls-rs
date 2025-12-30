// Generated macro for test_pop_empty (function)
macro_rules! Depcrate_teststest_pop_empty {
() => {
// Module: crate::tests
// Provides: {"test_pop_empty"}
// Dependencies: {}
# [test_case (CompactString :: from ("") ; "inline")] # [test_case (CompactString :: const_new ("") ; "static_str")] fn test_pop_empty (mut compact : CompactString) { let num_pops = 256 ; (0 .. num_pops) . for_each (| _ | { let ch = compact . pop () ; assert ! (ch . is_none ()) ; }) ; assert ! (compact . is_empty ()) ; assert_eq ! (compact , "") ; }
};
}
