// Generated macro for impl_1593 (impl)
macro_rules! Depcrate_geometry_quaternion_conversionimpl_1593 {
() => {
// Module: crate::geometry::quaternion_conversion
// Provides: {"impl_1593"}
// Dependencies: {}
impl < T : Scalar + Copy + PrimitiveSimdValue > From < [UnitQuaternion < T :: Element > ; 16] > for UnitQuaternion < T > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 16] > , T :: Element : Scalar + Copy , { # [inline] fn from (arr : [UnitQuaternion < T :: Element > ; 16]) -> Self { Self :: new_unchecked (Quaternion :: from ([arr [0] . into_inner () , arr [1] . into_inner () , arr [2] . into_inner () , arr [3] . into_inner () , arr [4] . into_inner () , arr [5] . into_inner () , arr [6] . into_inner () , arr [7] . into_inner () , arr [8] . into_inner () , arr [9] . into_inner () , arr [10] . into_inner () , arr [11] . into_inner () , arr [12] . into_inner () , arr [13] . into_inner () , arr [14] . into_inner () , arr [15] . into_inner () ,])) } }
};
}
