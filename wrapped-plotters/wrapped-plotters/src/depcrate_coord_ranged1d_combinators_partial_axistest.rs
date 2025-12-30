// Generated macro for test (module)
macro_rules! Depcrate_coord_ranged1d_combinators_partial_axistest {
() => {
// Module: crate::coord::ranged1d::combinators::partial_axis
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_make_partial_axis () { let r = make_partial_axis (20 .. 80 , 0.2 .. 0.8) . unwrap () ; assert_eq ! (r . size () , 101) ; assert_eq ! (r . range () , 0 .. 100) ; assert_eq ! (r . axis_pixel_range ((0 , 100)) , 20 .. 80) ; } }
};
}
