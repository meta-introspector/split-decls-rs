// Generated macro for impl_1738 (impl)
macro_rules! Depcrate_geometry_dual_quaternion_conversionimpl_1738 {
() => {
// Module: crate::geometry::dual_quaternion_conversion
// Provides: {"impl_1738"}
// Dependencies: {}
impl < T1 , T2 > SubsetOf < DualQuaternion < T2 > > for DualQuaternion < T1 > where T1 : SimdRealField , T2 : SimdRealField + SupersetOf < T1 > , { # [inline] fn to_superset (& self) -> DualQuaternion < T2 > { DualQuaternion :: from_real_and_dual (self . real . to_superset () , self . dual . to_superset ()) } # [inline] fn is_in_subset (dq : & DualQuaternion < T2 >) -> bool { crate :: is_convertible :: < _ , Vector4 < T1 > > (& dq . real . coords) && crate :: is_convertible :: < _ , Vector4 < T1 > > (& dq . dual . coords) } # [inline] fn from_superset_unchecked (dq : & DualQuaternion < T2 >) -> Self { DualQuaternion :: from_real_and_dual (dq . real . to_subset_unchecked () , dq . dual . to_subset_unchecked () ,) } }
};
}
