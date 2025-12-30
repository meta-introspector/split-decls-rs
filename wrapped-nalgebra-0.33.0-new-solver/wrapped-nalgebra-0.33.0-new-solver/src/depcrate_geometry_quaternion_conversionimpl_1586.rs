// Generated macro for impl_1586 (impl)
macro_rules! Depcrate_geometry_quaternion_conversionimpl_1586 {
() => {
// Module: crate::geometry::quaternion_conversion
// Provides: {"impl_1586"}
// Dependencies: {}
impl < T : Scalar + PrimitiveSimdValue > From < [Quaternion < T :: Element > ; 2] > for Quaternion < T > where T : From < [< T as SimdValue > :: Element ; 2] > , T :: Element : Scalar + Copy , { # [inline] fn from (arr : [Quaternion < T :: Element > ; 2]) -> Self { Self :: from (Vector4 :: from ([arr [0] . coords , arr [1] . coords])) } }
};
}
