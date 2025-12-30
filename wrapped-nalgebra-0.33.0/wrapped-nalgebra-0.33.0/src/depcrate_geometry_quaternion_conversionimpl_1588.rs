// Generated macro for impl_1588 (impl)
macro_rules! Depcrate_geometry_quaternion_conversionimpl_1588 {
() => {
// Module: crate::geometry::quaternion_conversion
// Provides: {"impl_1588"}
// Dependencies: {}
impl < T : Scalar + PrimitiveSimdValue > From < [Quaternion < T :: Element > ; 8] > for Quaternion < T > where T : From < [< T as SimdValue > :: Element ; 8] > , T :: Element : Scalar + Copy , { # [inline] fn from (arr : [Quaternion < T :: Element > ; 8]) -> Self { Self :: from (Vector4 :: from ([arr [0] . coords , arr [1] . coords , arr [2] . coords , arr [3] . coords , arr [4] . coords , arr [5] . coords , arr [6] . coords , arr [7] . coords ,])) } }
};
}
