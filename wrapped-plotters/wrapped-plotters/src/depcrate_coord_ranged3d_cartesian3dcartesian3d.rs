// Generated macro for Cartesian3d (struct)
macro_rules! Depcrate_coord_ranged3d_cartesian3dCartesian3d {
() => {
// Module: crate::coord::ranged3d::cartesian3d
// Provides: {"Cartesian3d"}
// Dependencies: {}
# [doc = " A 3D cartesian coordinate system"] # [derive (Clone)] pub struct Cartesian3d < X : Ranged , Y : Ranged , Z : Ranged > { pub (crate) logic_x : X , pub (crate) logic_y : Y , pub (crate) logic_z : Z , coord_size : (i32 , i32 , i32) , projection : ProjectionMatrix , }
};
}
