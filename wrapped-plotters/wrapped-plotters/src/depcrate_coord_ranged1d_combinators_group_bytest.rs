// Generated macro for test (module)
macro_rules! Depcrate_coord_ranged1d_combinators_group_bytest {
() => {
// Module: crate::coord::ranged1d::combinators::group_by
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_group_by () { let coord = (0 .. 100) . group_by (10) ; assert_eq ! (coord . size () , 11) ; for (idx , val) in (0 ..) . zip (coord . values ()) { assert_eq ! (val , idx * 10) ; assert_eq ! (coord . from_index (idx as usize) , Some (val)) ; } } }
};
}
