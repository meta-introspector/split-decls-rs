// Generated macro for Cartesian2d (struct)
macro_rules! Depcrate_coord_ranged2d_cartesianCartesian2d {
() => {
// Module: crate::coord::ranged2d::cartesian
// Provides: {"Cartesian2d"}
// Dependencies: {}
# [doc = " A 2D Cartesian coordinate system described by two 1D ranged coordinate specs."] # [derive (Clone)] pub struct Cartesian2d < X : Ranged , Y : Ranged > { logic_x : X , logic_y : Y , back_x : (i32 , i32) , back_y : (i32 , i32) , }
};
}
