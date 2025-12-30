// Generated macro for impl_1439 (impl)
macro_rules! Depcrate_geometry_rotation_conversionimpl_1439 {
() => {
// Module: crate::geometry::rotation_conversion
// Provides: {"impl_1439"}
// Dependencies: {}
impl < T : RealField > From < Rotation2 < T > > for Matrix3 < T > { # [inline] fn from (q : Rotation2 < T >) -> Self { q . to_homogeneous () } }
};
}
