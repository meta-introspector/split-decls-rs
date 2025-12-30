// Generated macro for check_either (function)
macro_rules! Depcrate_iter_testcheck_either {
() => {
// Module: crate::iter::test
// Provides: {"check_either"}
// Dependencies: {}
# [test] fn check_either () { type I = crate :: vec :: IntoIter < i32 > ; type E = Either < I , I > ; let v : Vec < i32 > = (0 .. 1024) . collect () ; let left : E = Either :: Left (v . clone () . into_par_iter ()) ; assert ! (left . eq (v . clone ())) ; let right : E = Either :: Right (v . clone () . into_par_iter ()) ; assert ! (right . eq (v . clone ())) ; let left : E = Either :: Left (v . clone () . into_par_iter ()) ; assert ! (left . enumerate () . eq (v . into_par_iter () . enumerate ())) ; }
};
}
