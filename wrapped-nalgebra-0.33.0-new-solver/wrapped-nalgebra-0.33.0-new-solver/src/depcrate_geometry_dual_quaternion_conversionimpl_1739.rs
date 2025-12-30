// Generated macro for impl_1739 (impl)
macro_rules! Depcrate_geometry_dual_quaternion_conversionimpl_1739 {
() => {
// Module: crate::geometry::dual_quaternion_conversion
// Provides: {"impl_1739"}
// Dependencies: {}
impl < T1 , T2 > SubsetOf < UnitDualQuaternion < T2 > > for UnitDualQuaternion < T1 > where T1 : SimdRealField , T2 : SimdRealField + SupersetOf < T1 > , { # [inline] fn to_superset (& self) -> UnitDualQuaternion < T2 > { UnitDualQuaternion :: new_unchecked (self . as_ref () . to_superset ()) } # [inline] fn is_in_subset (dq : & UnitDualQuaternion < T2 >) -> bool { crate :: is_convertible :: < _ , DualQuaternion < T1 > > (dq . as_ref ()) } # [inline] fn from_superset_unchecked (dq : & UnitDualQuaternion < T2 >) -> Self { Self :: new_unchecked (crate :: convert_ref_unchecked (dq . as_ref ())) } }
};
}
