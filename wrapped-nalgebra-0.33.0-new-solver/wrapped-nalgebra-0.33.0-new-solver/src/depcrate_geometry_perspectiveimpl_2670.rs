// Generated macro for impl_2670 (impl)
macro_rules! Depcrate_geometry_perspectiveimpl_2670 {
() => {
// Module: crate::geometry::perspective
// Provides: {"impl_2670"}
// Dependencies: {}
impl < T : RealField > From < Perspective3 < T > > for Matrix4 < T > { # [inline] fn from (pers : Perspective3 < T >) -> Self { pers . into_inner () } }
};
}
