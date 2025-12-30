// Generated macro for check_update (function)
macro_rules! Depcrate_iter_testcheck_update {
() => {
// Module: crate::iter::test
// Provides: {"check_update"}
// Dependencies: {}
# [test] fn check_update () { let mut v : Vec < Vec < _ > > = vec ! [vec ! [1] , vec ! [3 , 2 , 1]] ; v . par_iter_mut () . update (| v | v . push (0)) . for_each (| _ | ()) ; assert_eq ! (v , vec ! [vec ! [1 , 0] , vec ! [3 , 2 , 1 , 0]]) ; }
};
}
