// Generated macro for min_max (function)
macro_rules! Depcrate_iter_testmin_max {
() => {
// Module: crate::iter::test
// Provides: {"min_max"}
// Dependencies: {}
# [test] fn min_max () { let rng = seeded_rng () ; let a : Vec < i32 > = rng . sample_iter (& StandardUniform) . take (1024) . collect () ; for i in 0 ..= a . len () { let slice = & a [.. i] ; assert_eq ! (slice . par_iter () . min () , slice . iter () . min ()) ; assert_eq ! (slice . par_iter () . max () , slice . iter () . max ()) ; } }
};
}
