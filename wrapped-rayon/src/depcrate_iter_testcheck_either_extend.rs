// Generated macro for check_either_extend (function)
macro_rules! Depcrate_iter_testcheck_either_extend {
() => {
// Module: crate::iter::test
// Provides: {"check_either_extend"}
// Dependencies: {}
# [test] fn check_either_extend () { type E = Either < Vec < i32 > , HashSet < i32 > > ; let v : Vec < i32 > = (0 .. 1024) . collect () ; let mut left : E = Either :: Left (vec ! []) ; left . par_extend (v . clone ()) ; assert_eq ! (left . as_ref () , Either :: Left (& v)) ; let mut right : E = Either :: Right (HashSet :: default ()) ; right . par_extend (v . clone ()) ; assert_eq ! (right , Either :: Right (v . iter () . cloned () . collect ())) ; }
};
}
