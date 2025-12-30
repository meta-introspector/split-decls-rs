// Generated macro for impl_1740 (impl)
macro_rules! Depcrate_geometry_dual_quaternion_conversionimpl_1740 {
() => {
// Module: crate::geometry::dual_quaternion_conversion
// Provides: {"impl_1740"}
// Dependencies: {}
impl < T1 , T2 > SubsetOf < Isometry3 < T2 > > for UnitDualQuaternion < T1 > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , { # [inline] fn to_superset (& self) -> Isometry3 < T2 > { let dq : UnitDualQuaternion < T2 > = self . to_superset () ; let iso = dq . to_isometry () ; crate :: convert_unchecked (iso) } # [inline] fn is_in_subset (iso : & Isometry3 < T2 >) -> bool { crate :: is_convertible :: < _ , UnitQuaternion < T1 > > (& iso . rotation) && crate :: is_convertible :: < _ , Translation3 < T1 > > (& iso . translation) } # [inline] fn from_superset_unchecked (iso : & Isometry3 < T2 >) -> Self { let dq = UnitDualQuaternion :: < T2 > :: from_isometry (iso) ; crate :: convert_unchecked (dq) } }
};
}
