// Generated macro for iter_test (module)
macro_rules! Depcrate_output_treeiter_test {
() => {
// Module: crate::output::tree
// Provides: {"iter_test"}
// Dependencies: {}
# [cfg (test)] mod iter_test { use super :: * ; # [test] fn test_iteration () { let foos = & ["first" , "middle" , "last"] ; let mut iter = TreeDepth :: root () . iterate_over (foos . iter ()) ; let next = iter . next () . unwrap () ; assert_eq ! (& "first" , next . 1) ; assert ! (! next . 0 . last) ; let next = iter . next () . unwrap () ; assert_eq ! (& "middle" , next . 1) ; assert ! (! next . 0 . last) ; let next = iter . next () . unwrap () ; assert_eq ! (& "last" , next . 1) ; assert ! (next . 0 . last) ; assert ! (iter . next () . is_none ()) ; } # [test] fn test_empty () { let nothing : & [usize] = & [] ; let mut iter = TreeDepth :: root () . iterate_over (nothing . iter ()) ; assert ! (iter . next () . is_none ()) ; } }
};
}
