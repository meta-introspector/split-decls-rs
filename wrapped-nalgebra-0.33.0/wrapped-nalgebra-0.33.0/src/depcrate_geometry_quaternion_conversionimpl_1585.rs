// Generated macro for impl_1585 (impl)
macro_rules! Depcrate_geometry_quaternion_conversionimpl_1585 {
() => {
// Module: crate::geometry::quaternion_conversion
// Provides: {"impl_1585"}
// Dependencies: {}
impl < T : Scalar > From < [T ; 4] > for Quaternion < T > { # [inline] fn from (coords : [T ; 4]) -> Self { Self { coords : coords . into () , } } }
};
}
