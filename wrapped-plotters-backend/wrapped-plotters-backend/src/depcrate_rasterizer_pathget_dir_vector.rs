// Generated macro for get_dir_vector (function)
macro_rules! Depcrate_rasterizer_pathget_dir_vector {
() => {
// Module: crate::rasterizer::path
// Provides: {"get_dir_vector"}
// Dependencies: {}
fn get_dir_vector (from : BackendCoord , to : BackendCoord , flag : bool) -> ((f64 , f64) , (f64 , f64)) { let v = (i64 :: from (to . 0 - from . 0) , i64 :: from (to . 1 - from . 1)) ; let l = ((v . 0 * v . 0 + v . 1 * v . 1) as f64) . sqrt () ; let v = (v . 0 as f64 / l , v . 1 as f64 / l) ; if flag { (v , (v . 1 , - v . 0)) } else { (v , (- v . 1 , v . 0)) } }
};
}
