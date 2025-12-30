// Generated macro for test_to_satisfiable_range_suffix (function)
macro_rules! Depcrate_common_rangetest_to_satisfiable_range_suffix {
() => {
// Module: crate::common::range
// Provides: {"test_to_satisfiable_range_suffix"}
// Dependencies: {}
# [test] fn test_to_satisfiable_range_suffix () { let range = super :: test_decode :: < Range > (& ["bytes=-100"]) . unwrap () ; let bounds = range . satisfiable_ranges (350) . next () . unwrap () ; assert_eq ! (bounds , (Bound :: Included (250) , Bound :: Unbounded)) ; }
};
}
