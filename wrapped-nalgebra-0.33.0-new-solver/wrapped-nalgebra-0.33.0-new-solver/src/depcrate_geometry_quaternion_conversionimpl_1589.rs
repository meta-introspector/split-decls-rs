// Generated macro for impl_1589 (impl)
macro_rules! Depcrate_geometry_quaternion_conversionimpl_1589 {
() => {
// Module: crate::geometry::quaternion_conversion
// Provides: {"impl_1589"}
// Dependencies: {}
impl < T : Scalar + PrimitiveSimdValue > From < [Quaternion < T :: Element > ; 16] > for Quaternion < T > where T : From < [< T as SimdValue > :: Element ; 16] > , T :: Element : Scalar + Copy , { # [inline] fn from (arr : [Quaternion < T :: Element > ; 16]) -> Self { Self :: from (Vector4 :: from ([arr [0] . coords , arr [1] . coords , arr [2] . coords , arr [3] . coords , arr [4] . coords , arr [5] . coords , arr [6] . coords , arr [7] . coords , arr [8] . coords , arr [9] . coords , arr [10] . coords , arr [11] . coords , arr [12] . coords , arr [13] . coords , arr [14] . coords , arr [15] . coords ,])) } }
};
}
