// Generated macro for min_max_by_key (function)
macro_rules! Depcrate_iter_testmin_max_by_key {
() => {
// Module: crate::iter::test
// Provides: {"min_max_by_key"}
// Dependencies: {}
# [test] fn min_max_by_key () { let rng = seeded_rng () ; let r : Vec < i32 > = rng . sample_iter (& StandardUniform) . take (512) . collect () ; let a : Vec < (i32 , u16) > = r . iter () . chain (& r) . cloned () . zip (0 ..) . collect () ; for i in 0 ..= a . len () { let slice = & a [.. i] ; assert_eq ! (slice . par_iter () . min_by_key (| x | x . 0) , slice . iter () . min_by_key (| x | x . 0)) ; assert_eq ! (slice . par_iter () . max_by_key (| x | x . 0) , slice . iter () . max_by_key (| x | x . 0)) ; } }
};
}
