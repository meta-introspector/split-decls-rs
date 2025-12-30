// Generated macro for impl_1442 (impl)
macro_rules! Depcrate_geometry_rotation_conversionimpl_1442 {
() => {
// Module: crate::geometry::rotation_conversion
// Provides: {"impl_1442"}
// Dependencies: {}
impl < T : RealField > From < Rotation3 < T > > for Matrix3 < T > { # [inline] fn from (q : Rotation3 < T >) -> Self { q . into_inner () } }
};
}
