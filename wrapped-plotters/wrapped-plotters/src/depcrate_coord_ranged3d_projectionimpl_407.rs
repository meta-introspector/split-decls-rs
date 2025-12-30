// Generated macro for impl_407 (impl)
macro_rules! Depcrate_coord_ranged3d_projectionimpl_407 {
() => {
// Module: crate::coord::ranged3d::projection
// Provides: {"impl_407"}
// Dependencies: {}
impl Mul < (f64 , f64 , f64) > for ProjectionMatrix { type Output = (i32 , i32) ; fn mul (self , (x , y , z) : (f64 , f64 , f64)) -> (i32 , i32) { let m = self . 0 ; ((x * m [0] [0] + y * m [0] [1] + z * m [0] [2] + m [0] [3]) as i32 , (x * m [1] [0] + y * m [1] [1] + z * m [1] [2] + m [1] [3]) as i32 ,) } }
};
}
