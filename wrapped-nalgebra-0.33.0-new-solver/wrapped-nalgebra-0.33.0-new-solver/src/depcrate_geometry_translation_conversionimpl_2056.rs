// Generated macro for impl_2056 (impl)
macro_rules! Depcrate_geometry_translation_conversionimpl_2056 {
() => {
// Module: crate::geometry::translation_conversion
// Provides: {"impl_2056"}
// Dependencies: {}
impl < T1 , T2 > SubsetOf < UnitDualQuaternion < T2 > > for Translation3 < T1 > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , { # [inline] fn to_superset (& self) -> UnitDualQuaternion < T2 > { let dq = UnitDualQuaternion :: < T1 > :: from_parts (self . clone () , UnitQuaternion :: identity ()) ; dq . to_superset () } # [inline] fn is_in_subset (dq : & UnitDualQuaternion < T2 >) -> bool { crate :: is_convertible :: < _ , Translation < T1 , 3 > > (& dq . translation ()) && dq . rotation () == UnitQuaternion :: identity () } # [inline] fn from_superset_unchecked (dq : & UnitDualQuaternion < T2 >) -> Self { let dq : UnitDualQuaternion < T1 > = crate :: convert_ref_unchecked (dq) ; dq . translation () } }
};
}
