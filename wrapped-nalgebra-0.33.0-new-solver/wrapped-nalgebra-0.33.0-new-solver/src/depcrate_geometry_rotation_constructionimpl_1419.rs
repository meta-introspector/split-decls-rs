// Generated macro for impl_1419 (impl)
macro_rules! Depcrate_geometry_rotation_constructionimpl_1419 {
() => {
// Module: crate::geometry::rotation_construction
// Provides: {"impl_1419"}
// Dependencies: {}
# [doc = " # Identity"] impl < T , const D : usize > Rotation < T , D > where T : Scalar + Zero + One , { # [doc = " Creates a new square identity rotation of the given `dimension`."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::{Rotation2, Rotation3};"] # [doc = " # use nalgebra::Vector3;"] # [doc = " let rot1 = Rotation2::identity();"] # [doc = " let rot2 = Rotation2::new(std::f32::consts::FRAC_PI_2);"] # [doc = ""] # [doc = " assert_eq!(rot1 * rot2, rot2);"] # [doc = " assert_eq!(rot2 * rot1, rot2);"] # [doc = ""] # [doc = " let rot1 = Rotation3::identity();"] # [doc = " let rot2 = Rotation3::from_axis_angle(&Vector3::z_axis(), std::f32::consts::FRAC_PI_2);"] # [doc = ""] # [doc = " assert_eq!(rot1 * rot2, rot2);"] # [doc = " assert_eq!(rot2 * rot1, rot2);"] # [doc = " ```"] # [inline] pub fn identity () -> Rotation < T , D > { Self :: from_matrix_unchecked (SMatrix :: < T , D , D > :: identity ()) } }
};
}
