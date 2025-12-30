// Generated macro for check_for_each_with (function)
macro_rules! Depcrate_iter_testcheck_for_each_with {
() => {
// Module: crate::iter::test
// Provides: {"check_for_each_with"}
// Dependencies: {}
# [test] fn check_for_each_with () { let (sender , receiver) = mpsc :: channel () ; let a : HashSet < _ > = (0 .. 1024) . collect () ; a . par_iter () . cloned () . for_each_with (sender , | s , i | s . send (i) . unwrap ()) ; let b : HashSet < _ > = receiver . iter () . collect () ; assert_eq ! (a , b) ; }
};
}
