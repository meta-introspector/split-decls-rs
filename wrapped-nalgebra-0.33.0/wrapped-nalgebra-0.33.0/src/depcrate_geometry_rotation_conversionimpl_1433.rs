// Generated macro for impl_1433 (impl)
macro_rules! Depcrate_geometry_rotation_conversionimpl_1433 {
() => {
// Module: crate::geometry::rotation_conversion
// Provides: {"impl_1433"}
// Dependencies: {}
impl < T1 , T2 > SubsetOf < UnitDualQuaternion < T2 > > for Rotation3 < T1 > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , { # [inline] fn to_superset (& self) -> UnitDualQuaternion < T2 > { let q = UnitQuaternion :: < T1 > :: from_rotation_matrix (self) ; let dq = UnitDualQuaternion :: from_rotation (q) ; dq . to_superset () } # [inline] fn is_in_subset (dq : & UnitDualQuaternion < T2 >) -> bool { crate :: is_convertible :: < _ , UnitQuaternion < T1 > > (& dq . rotation ()) && dq . translation () . vector . is_zero () } # [inline] fn from_superset_unchecked (dq : & UnitDualQuaternion < T2 >) -> Self { let dq : UnitDualQuaternion < T1 > = crate :: convert_ref_unchecked (dq) ; dq . rotation () . to_rotation_matrix () } }
};
}
