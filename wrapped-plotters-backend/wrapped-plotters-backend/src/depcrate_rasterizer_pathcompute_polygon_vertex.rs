// Generated macro for compute_polygon_vertex (function)
macro_rules! Depcrate_rasterizer_pathcompute_polygon_vertex {
() => {
// Module: crate::rasterizer::path
// Provides: {"compute_polygon_vertex"}
// Dependencies: {}
fn compute_polygon_vertex (triple : & [BackendCoord ; 3] , d : f64 , buf : & mut Vec < BackendCoord >) { buf . clear () ; let (a_t , a_n) = get_dir_vector (triple [0] , triple [1] , false) ; let (b_t , b_n) = get_dir_vector (triple [2] , triple [1] , true) ; let a_p = (f64 :: from (triple [1] . 0) + d * a_n . 0 , f64 :: from (triple [1] . 1) + d * a_n . 1 ,) ; let b_p = (f64 :: from (triple [1] . 0) + d * b_n . 0 , f64 :: from (triple [1] . 1) + d * b_n . 1 ,) ; if (a_t . 1 * b_t . 0 - a_t . 0 * b_t . 1) . abs () <= f64 :: EPSILON { buf . push ((a_p . 0 as i32 , a_p . 1 as i32)) ; return ; } let a0 = a_t . 0 ; let b0 = - b_t . 0 ; let c0 = b_p . 0 - a_p . 0 ; let a1 = a_t . 1 ; let b1 = - b_t . 1 ; let c1 = b_p . 1 - a_p . 1 ; let u = (c0 * b1 - c1 * b0) / (a0 * b1 - a1 * b0) ; let x = a_p . 0 + u * a_t . 0 ; let y = a_p . 1 + u * a_t . 1 ; let cross_product = a_t . 0 * b_t . 1 - a_t . 1 * b_t . 0 ; if (cross_product < 0.0 && d < 0.0) || (cross_product > 0.0 && d > 0.0) { let dist_square = (x - triple [1] . 0 as f64) . powi (2) + (y - triple [1] . 1 as f64) . powi (2) ; if dist_square > d * d * 16.0 { buf . push ((a_p . 0 . round () as i32 , a_p . 1 . round () as i32)) ; buf . push ((b_p . 0 . round () as i32 , b_p . 1 . round () as i32)) ; return ; } } buf . push ((x . round () as i32 , y . round () as i32)) ; }
};
}
