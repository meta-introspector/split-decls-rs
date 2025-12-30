// Generated macro for impl_1591 (impl)
macro_rules! Depcrate_geometry_quaternion_conversionimpl_1591 {
() => {
// Module: crate::geometry::quaternion_conversion
// Provides: {"impl_1591"}
// Dependencies: {}
impl < T : Scalar + Copy + PrimitiveSimdValue > From < [UnitQuaternion < T :: Element > ; 4] > for UnitQuaternion < T > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 4] > , T :: Element : Scalar + Copy , { # [inline] fn from (arr : [UnitQuaternion < T :: Element > ; 4]) -> Self { Self :: new_unchecked (Quaternion :: from ([arr [0] . into_inner () , arr [1] . into_inner () , arr [2] . into_inner () , arr [3] . into_inner () ,])) } }
};
}
