// Generated macro for impl_1441 (impl)
macro_rules! Depcrate_geometry_rotation_conversionimpl_1441 {
() => {
// Module: crate::geometry::rotation_conversion
// Provides: {"impl_1441"}
// Dependencies: {}
impl < T : RealField > From < Rotation3 < T > > for Matrix4 < T > { # [inline] fn from (q : Rotation3 < T >) -> Self { q . to_homogeneous () } }
};
}
