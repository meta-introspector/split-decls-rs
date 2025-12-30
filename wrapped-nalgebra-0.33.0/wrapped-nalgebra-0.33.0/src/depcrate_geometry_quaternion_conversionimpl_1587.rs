// Generated macro for impl_1587 (impl)
macro_rules! Depcrate_geometry_quaternion_conversionimpl_1587 {
() => {
// Module: crate::geometry::quaternion_conversion
// Provides: {"impl_1587"}
// Dependencies: {}
impl < T : Scalar + PrimitiveSimdValue > From < [Quaternion < T :: Element > ; 4] > for Quaternion < T > where T : From < [< T as SimdValue > :: Element ; 4] > , T :: Element : Scalar + Copy , { # [inline] fn from (arr : [Quaternion < T :: Element > ; 4]) -> Self { Self :: from (Vector4 :: from ([arr [0] . coords , arr [1] . coords , arr [2] . coords , arr [3] . coords ,])) } }
};
}
