// Generated macro for impl_1490 (impl)
macro_rules! Depcrate_geometry_rotation_specializationimpl_1490 {
() => {
// Module: crate::geometry::rotation_specialization
// Provides: {"impl_1490"}
// Dependencies: {}
# [doc = " # Construction from a 2D rotation angle"] impl < T : SimdRealField > Rotation2 < T > { # [doc = " Builds a 2 dimensional rotation matrix from an angle in radian."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate approx;"] # [doc = " # use std::f32;"] # [doc = " # use nalgebra::{Rotation2, Point2};"] # [doc = " let rot = Rotation2::new(f32::consts::FRAC_PI_2);"] # [doc = ""] # [doc = " assert_relative_eq!(rot * Point2::new(3.0, 4.0), Point2::new(-4.0, 3.0));"] # [doc = " ```"] pub fn new (angle : T) -> Self { let (sia , coa) = angle . simd_sin_cos () ; Self :: from_matrix_unchecked (Matrix2 :: new (coa . clone () , - sia . clone () , sia , coa)) } # [doc = " Builds a 2 dimensional rotation matrix from an angle in radian wrapped in a 1-dimensional vector."] # [doc = ""] # [doc = ""] # [doc = " This is generally used in the context of generic programming. Using"] # [doc = " the `::new(angle)` method instead is more common."] # [inline] pub fn from_scaled_axis < SB : Storage < T , U1 > > (axisangle : Vector < T , U1 , SB >) -> Self { Self :: new (axisangle [0] . clone ()) } }
};
}
