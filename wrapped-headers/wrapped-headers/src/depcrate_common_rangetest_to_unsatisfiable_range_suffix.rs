// Generated macro for test_to_unsatisfiable_range_suffix (function)
macro_rules! Depcrate_common_rangetest_to_unsatisfiable_range_suffix {
() => {
// Module: crate::common::range
// Provides: {"test_to_unsatisfiable_range_suffix"}
// Dependencies: {}
# [test] fn test_to_unsatisfiable_range_suffix () { let range = super :: test_decode :: < Range > (& ["bytes=-350"]) . unwrap () ; let bounds = range . satisfiable_ranges (100) . next () ; assert_eq ! (bounds , None) ; }
};
}
