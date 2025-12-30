// Generated macro for check_fold_with (function)
macro_rules! Depcrate_iter_testcheck_fold_with {
() => {
// Module: crate::iter::test
// Provides: {"check_fold_with"}
// Dependencies: {}
# [test] fn check_fold_with () { let (sender , receiver) = mpsc :: channel () ; let a : HashSet < _ > = (0 .. 1024) . collect () ; a . par_iter () . cloned () . fold_with (sender , | s , i | { s . send (i) . unwrap () ; s }) . count () ; let b : HashSet < _ > = receiver . iter () . collect () ; assert_eq ! (a , b) ; }
};
}
