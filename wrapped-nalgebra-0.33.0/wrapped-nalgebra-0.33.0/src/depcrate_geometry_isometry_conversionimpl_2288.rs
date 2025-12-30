// Generated macro for impl_2288 (impl)
macro_rules! Depcrate_geometry_isometry_conversionimpl_2288 {
() => {
// Module: crate::geometry::isometry_conversion
// Provides: {"impl_2288"}
// Dependencies: {}
impl < T1 , T2 > SubsetOf < UnitDualQuaternion < T2 > > for Isometry3 < T1 > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , { # [inline] fn to_superset (& self) -> UnitDualQuaternion < T2 > { let dq = UnitDualQuaternion :: < T1 > :: from_isometry (self) ; dq . to_superset () } # [inline] fn is_in_subset (dq : & UnitDualQuaternion < T2 >) -> bool { crate :: is_convertible :: < _ , UnitQuaternion < T1 > > (& dq . rotation ()) && crate :: is_convertible :: < _ , Translation < T1 , 3 > > (& dq . translation ()) } # [inline] fn from_superset_unchecked (dq : & UnitDualQuaternion < T2 >) -> Self { let dq : UnitDualQuaternion < T1 > = crate :: convert_ref_unchecked (dq) ; dq . to_isometry () } }
};
}
