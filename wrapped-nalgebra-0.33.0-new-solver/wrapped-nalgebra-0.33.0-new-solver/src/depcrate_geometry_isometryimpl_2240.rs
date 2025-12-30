// Generated macro for impl_2240 (impl)
macro_rules! Depcrate_geometry_isometryimpl_2240 {
() => {
// Module: crate::geometry::isometry
// Provides: {"impl_2240"}
// Dependencies: {}
# [doc = " # From the translation and rotation parts"] impl < T : Scalar , R : AbstractRotation < T , D > , const D : usize > Isometry < T , R , D > { # [doc = " Creates a new isometry from its rotational and translational parts."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate approx;"] # [doc = " # use std::f32;"] # [doc = " # use nalgebra::{Isometry3, Translation3, UnitQuaternion, Vector3, Point3};"] # [doc = " let tra = Translation3::new(0.0, 0.0, 3.0);"] # [doc = " let rot = UnitQuaternion::from_scaled_axis(Vector3::y() * f32::consts::PI);"] # [doc = " let iso = Isometry3::from_parts(tra, rot);"] # [doc = ""] # [doc = " assert_relative_eq!(iso * Point3::new(1.0, 2.0, 3.0), Point3::new(-1.0, 2.0, 0.0), epsilon = 1.0e-6);"] # [doc = " ```"] # [inline] pub fn from_parts (translation : Translation < T , D > , rotation : R) -> Self { Self { rotation , translation , } } }
};
}
