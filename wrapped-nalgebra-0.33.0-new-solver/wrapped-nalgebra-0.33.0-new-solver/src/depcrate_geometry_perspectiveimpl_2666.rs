// Generated macro for impl_2666 (impl)
macro_rules! Depcrate_geometry_perspectiveimpl_2666 {
() => {
// Module: crate::geometry::perspective
// Provides: {"impl_2666"}
// Dependencies: {}
impl < T > Perspective3 < T > { # [doc = " Wraps the given matrix to interpret it as a 3D perspective matrix."] # [doc = ""] # [doc = " It is not checked whether or not the given matrix actually represents a perspective"] # [doc = " projection."] # [inline] pub const fn from_matrix_unchecked (matrix : Matrix4 < T >) -> Self { Self { matrix } } }
};
}
