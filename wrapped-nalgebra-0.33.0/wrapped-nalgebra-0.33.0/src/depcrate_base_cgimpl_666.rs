// Generated macro for impl_666 (impl)
macro_rules! Depcrate_base_cgimpl_666 {
() => {
// Module: crate::base::cg
// Provides: {"impl_666"}
// Dependencies: {}
# [doc = " # 2D transformations as a Matrix3"] impl < T : RealField > Matrix3 < T > { # [doc = " Builds a 2 dimensional homogeneous rotation matrix from an angle in radian."] # [inline] pub fn new_rotation (angle : T) -> Self { Rotation2 :: new (angle) . to_homogeneous () } # [doc = " Creates a new homogeneous matrix that applies a scaling factor for each dimension with respect to point."] # [doc = ""] # [doc = " Can be used to implement `zoom_to` functionality."] # [inline] pub fn new_nonuniform_scaling_wrt_point (scaling : & Vector2 < T > , pt : & Point2 < T >) -> Self { let zero = T :: zero () ; let one = T :: one () ; Matrix3 :: new (scaling . x . clone () , zero . clone () , pt . x . clone () - pt . x . clone () * scaling . x . clone () , zero . clone () , scaling . y . clone () , pt . y . clone () - pt . y . clone () * scaling . y . clone () , zero . clone () , zero , one ,) } }
};
}
