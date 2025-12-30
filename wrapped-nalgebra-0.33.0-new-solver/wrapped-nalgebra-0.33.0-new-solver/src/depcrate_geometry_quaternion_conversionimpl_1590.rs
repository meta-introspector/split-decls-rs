// Generated macro for impl_1590 (impl)
macro_rules! Depcrate_geometry_quaternion_conversionimpl_1590 {
() => {
// Module: crate::geometry::quaternion_conversion
// Provides: {"impl_1590"}
// Dependencies: {}
impl < T : Scalar + Copy + PrimitiveSimdValue > From < [UnitQuaternion < T :: Element > ; 2] > for UnitQuaternion < T > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 2] > , T :: Element : Scalar + Copy , { # [inline] fn from (arr : [UnitQuaternion < T :: Element > ; 2]) -> Self { Self :: new_unchecked (Quaternion :: from ([arr [0] . into_inner () , arr [1] . into_inner ()])) } }
};
}
