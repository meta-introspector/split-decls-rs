// Generated macro for impl_2647 (impl)
macro_rules! Depcrate_geometry_orthographicimpl_2647 {
() => {
// Module: crate::geometry::orthographic
// Provides: {"impl_2647"}
// Dependencies: {}
impl < T : RealField > From < Orthographic3 < T > > for Matrix4 < T > { # [inline] fn from (orth : Orthographic3 < T >) -> Self { orth . into_inner () } }
};
}
