// Generated macro for check_split (function)
macro_rules! Depcrate_iter_testcheck_split {
() => {
// Module: crate::iter::test
// Provides: {"check_split"}
// Dependencies: {}
# [test] fn check_split () { use std :: ops :: Range ; let a = (0 .. 1024) . into_par_iter () ; let b = split (0 .. 1024 , | Range { start , end } | { let mid = (end - start) / 2 ; if mid > start { (start .. mid , Some (mid .. end)) } else { (start .. end , None) } }) . flat_map (| range | range) ; assert_eq ! (a . collect ::< Vec < _ >> () , b . collect ::< Vec < _ >> ()) ; }
};
}
