// Generated macro for impl_1440 (impl)
macro_rules! Depcrate_geometry_rotation_conversionimpl_1440 {
() => {
// Module: crate::geometry::rotation_conversion
// Provides: {"impl_1440"}
// Dependencies: {}
impl < T : RealField > From < Rotation2 < T > > for Matrix2 < T > { # [inline] fn from (q : Rotation2 < T >) -> Self { q . into_inner () } }
};
}
