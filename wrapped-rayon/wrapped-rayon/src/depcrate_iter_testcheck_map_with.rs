// Generated macro for check_map_with (function)
macro_rules! Depcrate_iter_testcheck_map_with {
() => {
// Module: crate::iter::test
// Provides: {"check_map_with"}
// Dependencies: {}
# [test] fn check_map_with () { let (sender , receiver) = mpsc :: channel () ; let a : HashSet < _ > = (0 .. 1024) . collect () ; a . par_iter () . cloned () . map_with (sender , | s , i | s . send (i) . unwrap ()) . count () ; let b : HashSet < _ > = receiver . iter () . collect () ; assert_eq ! (a , b) ; }
};
}
