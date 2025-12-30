// Generated macro for impl_411 (impl)
macro_rules! Depcrate_coord_ranged3d_projectionimpl_411 {
() => {
// Module: crate::coord::ranged3d::projection
// Provides: {"impl_411"}
// Dependencies: {}
impl ProjectionMatrixBuilder { # [doc = " Creates a new, default projection matrix builder object."] pub fn new () -> Self { Self :: default () } # [doc = " Set the pivot point, which means the 3D coordinate \"before\" should be mapped into"] # [doc = " the 2D coordinatet \"after\""] pub fn set_pivot (& mut self , before : (i32 , i32 , i32) , after : (i32 , i32)) -> & mut Self { self . pivot_before = before ; self . pivot_after = after ; self } # [doc = " Build the matrix based on the configuration"] pub fn into_matrix (self) -> ProjectionMatrix { let mut ret = if self . pivot_before == (0 , 0 , 0) { ProjectionMatrix :: default () } else { let (x , y , z) = self . pivot_before ; ProjectionMatrix :: shift (- x as f64 , - y as f64 , - z as f64) * ProjectionMatrix :: default () } ; if self . yaw . abs () > 1e-20 { ret = ret * ProjectionMatrix :: rotate (0.0 , self . yaw , 0.0) ; } if self . pitch . abs () > 1e-20 { ret = ret * ProjectionMatrix :: rotate (self . pitch , 0.0 , 0.0) ; } if (self . scale - 1.0) . abs () > 1e-20 { ret = ret * ProjectionMatrix :: scale (self . scale) ; } if self . pivot_after != (0 , 0) { let (x , y) = self . pivot_after ; ret = ret * ProjectionMatrix :: shift (x as f64 , y as f64 , 0.0) ; } ret } }
};
}
