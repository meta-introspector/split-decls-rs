// Generated macro for par_iter_collect_btreeset (function)
macro_rules! Depcrate_iter_testpar_iter_collect_btreeset {
() => {
// Module: crate::iter::test
// Provides: {"par_iter_collect_btreeset"}
// Dependencies: {}
# [test] fn par_iter_collect_btreeset () { let a : Vec < i32 > = (0 .. 1024) . collect () ; let b : BTreeSet < i32 > = a . par_iter () . cloned () . collect () ; assert_eq ! (b . len () , 1024) ; }
};
}
