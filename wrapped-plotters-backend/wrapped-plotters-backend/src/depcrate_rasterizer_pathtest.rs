// Generated macro for test (module)
macro_rules! Depcrate_rasterizer_pathtest {
() => {
// Module: crate::rasterizer::path
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [doc = " Test for regression with respect to https://github.com/plotters-rs/plotters/issues/562"] # [test] fn test_no_inf_in_compute_polygon_vertex () { let path = [(335 , 386) , (338 , 326) , (340 , 286)] ; let mut buf = Vec :: new () ; compute_polygon_vertex (& path , 2.0 , buf . as_mut ()) ; assert ! (! buf . is_empty ()) ; let nani32 = f64 :: INFINITY as i32 ; assert ! (! buf . iter () . any (|& v | v . 0 == nani32 || v . 1 == nani32)) ; } # [doc = " Correct 90 degree turn to the right"] # [test] fn standard_corner () { let path = [(10 , 10) , (20 , 10) , (20 , 20)] ; let mut buf = Vec :: new () ; compute_polygon_vertex (& path , 2.0 , buf . as_mut ()) ; assert ! (! buf . is_empty ()) ; let buf2 = vec ! [(18 , 12)] ; assert_eq ! (buf , buf2) ; } }
};
}
