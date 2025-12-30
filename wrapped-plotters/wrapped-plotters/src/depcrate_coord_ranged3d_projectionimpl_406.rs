// Generated macro for impl_406 (impl)
macro_rules! Depcrate_coord_ranged3d_projectionimpl_406 {
() => {
// Module: crate::coord::ranged3d::projection
// Provides: {"impl_406"}
// Dependencies: {}
impl Mul < (i32 , i32 , i32) > for ProjectionMatrix { type Output = (i32 , i32) ; fn mul (self , (x , y , z) : (i32 , i32 , i32)) -> (i32 , i32) { let (x , y , z) = (x as f64 , y as f64 , z as f64) ; let m = self . 0 ; ((x * m [0] [0] + y * m [0] [1] + z * m [0] [2] + m [0] [3]) as i32 , (x * m [1] [0] + y * m [1] [1] + z * m [1] [2] + m [1] [3]) as i32 ,) } }
};
}
