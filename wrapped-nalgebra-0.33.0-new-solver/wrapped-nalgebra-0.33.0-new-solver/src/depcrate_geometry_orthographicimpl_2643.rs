// Generated macro for impl_2643 (impl)
macro_rules! Depcrate_geometry_orthographicimpl_2643 {
() => {
// Module: crate::geometry::orthographic
// Provides: {"impl_2643"}
// Dependencies: {}
impl < T > Orthographic3 < T > { # [doc = " Wraps the given matrix to interpret it as a 3D orthographic matrix."] # [doc = ""] # [doc = " It is not checked whether or not the given matrix actually represents an orthographic"] # [doc = " projection."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::{Orthographic3, Point3, Matrix4};"] # [doc = " let mat = Matrix4::new("] # [doc = "     2.0 / 9.0, 0.0,        0.0,         -11.0 / 9.0,"] # [doc = "     0.0,       2.0 / 18.0, 0.0,         -22.0 / 18.0,"] # [doc = "     0.0,       0.0,       -2.0 / 999.9, -1000.1 / 999.9,"] # [doc = "     0.0,       0.0,        0.0,         1.0"] # [doc = " );"] # [doc = " let proj = Orthographic3::from_matrix_unchecked(mat);"] # [doc = " assert_eq!(proj, Orthographic3::new(1.0, 10.0, 2.0, 20.0, 0.1, 1000.0));"] # [doc = " ```"] # [inline] pub const fn from_matrix_unchecked (matrix : Matrix4 < T >) -> Self { Self { matrix } } }
};
}
