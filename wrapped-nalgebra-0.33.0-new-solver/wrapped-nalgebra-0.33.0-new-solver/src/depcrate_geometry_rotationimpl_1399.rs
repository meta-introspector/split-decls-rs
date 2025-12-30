// Generated macro for impl_1399 (impl)
macro_rules! Depcrate_geometry_rotationimpl_1399 {
() => {
// Module: crate::geometry::rotation
// Provides: {"impl_1399"}
// Dependencies: {}
impl < T , const D : usize > Rotation < T , D > { # [doc = " Creates a new rotation from the given square matrix."] # [doc = ""] # [doc = " The matrix orthonormality is not checked."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::{Rotation2, Rotation3, Matrix2, Matrix3};"] # [doc = " # use std::f32;"] # [doc = " let mat = Matrix3::new(0.8660254, -0.5,      0.0,"] # [doc = "                        0.5,       0.8660254, 0.0,"] # [doc = "                        0.0,       0.0,       1.0);"] # [doc = " let rot = Rotation3::from_matrix_unchecked(mat);"] # [doc = ""] # [doc = " assert_eq!(*rot.matrix(), mat);"] # [doc = ""] # [doc = ""] # [doc = " let mat = Matrix2::new(0.8660254, -0.5,"] # [doc = "                        0.5,       0.8660254);"] # [doc = " let rot = Rotation2::from_matrix_unchecked(mat);"] # [doc = ""] # [doc = " assert_eq!(*rot.matrix(), mat);"] # [doc = " ```"] # [inline] pub const fn from_matrix_unchecked (matrix : SMatrix < T , D , D >) -> Self { Self { matrix } } }
};
}
