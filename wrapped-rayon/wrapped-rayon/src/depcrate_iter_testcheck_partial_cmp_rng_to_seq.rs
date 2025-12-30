// Generated macro for check_partial_cmp_rng_to_seq (function)
macro_rules! Depcrate_iter_testcheck_partial_cmp_rng_to_seq {
() => {
// Module: crate::iter::test
// Provides: {"check_partial_cmp_rng_to_seq"}
// Dependencies: {}
# [test] fn check_partial_cmp_rng_to_seq () { let mut rng = seeded_rng () ; let rng = & mut rng ; let a : Vec < i32 > = rng . sample_iter (& StandardUniform) . take (1024) . collect () ; let b : Vec < i32 > = rng . sample_iter (& StandardUniform) . take (1024) . collect () ; for i in 0 .. a . len () { let par_result = a [i ..] . par_iter () . partial_cmp (b [i ..] . par_iter ()) ; let seq_result = a [i ..] . iter () . partial_cmp (b [i ..] . iter ()) ; assert_eq ! (par_result , seq_result) ; } }
};
}
