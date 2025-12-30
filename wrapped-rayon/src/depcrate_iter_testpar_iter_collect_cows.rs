// Generated macro for par_iter_collect_cows (function)
macro_rules! Depcrate_iter_testpar_iter_collect_cows {
() => {
// Module: crate::iter::test
// Provides: {"par_iter_collect_cows"}
// Dependencies: {}
# [test] fn par_iter_collect_cows () { use std :: borrow :: Cow ; let s = "Fearless Concurrency with Rust" ; let a : Cow < '_ , [i32] > = (0 .. 1024) . collect () ; let b : Cow < '_ , [i32] > = a . par_iter () . cloned () . collect () ; assert_eq ! (a , b) ; let a : Cow < '_ , str > = s . chars () . collect () ; let b : Cow < '_ , str > = s . par_chars () . collect () ; assert_eq ! (a , b) ; let sw = s . split_whitespace () ; let psw = s . par_split_whitespace () ; let a : Cow < '_ , str > = sw . clone () . collect () ; let b : Cow < '_ , str > = psw . clone () . collect () ; assert_eq ! (a , b) ; let a : Cow < '_ , str > = sw . map (str :: to_owned) . collect () ; let b : Cow < '_ , str > = psw . map (str :: to_owned) . collect () ; assert_eq ! (a , b) ; let sw = s . split_whitespace () . map (OsStr :: new) ; let psw = s . par_split_whitespace () . map (OsStr :: new) ; let a : Cow < '_ , OsStr > = Cow :: Owned (sw . clone () . collect ()) ; let b : Cow < '_ , OsStr > = psw . clone () . collect () ; assert_eq ! (a , b) ; let a : Cow < '_ , OsStr > = Cow :: Owned (sw . map (OsStr :: to_owned) . collect ()) ; let b : Cow < '_ , OsStr > = psw . map (OsStr :: to_owned) . collect () ; assert_eq ! (a , b) ; }
};
}
