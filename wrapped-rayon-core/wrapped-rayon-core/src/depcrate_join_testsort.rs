// Generated macro for sort (function)
macro_rules! Depcrate_join_testsort {
() => {
// Module: crate::join::test
// Provides: {"sort"}
// Dependencies: {}
# [test] fn sort () { let rng = seeded_rng () ; let mut data : Vec < u32 > = rng . sample_iter (& StandardUniform) . take (6 * 1024) . collect () ; let mut sorted_data = data . clone () ; sorted_data . sort () ; quick_sort (& mut data) ; assert_eq ! (data , sorted_data) ; }
};
}
