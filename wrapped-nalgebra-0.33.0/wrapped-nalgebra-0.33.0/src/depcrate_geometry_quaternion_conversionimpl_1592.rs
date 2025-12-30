// Generated macro for impl_1592 (impl)
macro_rules! Depcrate_geometry_quaternion_conversionimpl_1592 {
() => {
// Module: crate::geometry::quaternion_conversion
// Provides: {"impl_1592"}
// Dependencies: {}
impl < T : Scalar + Copy + PrimitiveSimdValue > From < [UnitQuaternion < T :: Element > ; 8] > for UnitQuaternion < T > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 8] > , T :: Element : Scalar + Copy , { # [inline] fn from (arr : [UnitQuaternion < T :: Element > ; 8]) -> Self { Self :: new_unchecked (Quaternion :: from ([arr [0] . into_inner () , arr [1] . into_inner () , arr [2] . into_inner () , arr [3] . into_inner () , arr [4] . into_inner () , arr [5] . into_inner () , arr [6] . into_inner () , arr [7] . into_inner () ,])) } }
};
}
