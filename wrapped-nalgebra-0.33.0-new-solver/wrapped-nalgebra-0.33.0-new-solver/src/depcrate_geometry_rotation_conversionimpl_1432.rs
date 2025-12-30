// Generated macro for impl_1432 (impl)
macro_rules! Depcrate_geometry_rotation_conversionimpl_1432 {
() => {
// Module: crate::geometry::rotation_conversion
// Provides: {"impl_1432"}
// Dependencies: {}
impl < T1 , T2 > SubsetOf < UnitQuaternion < T2 > > for Rotation3 < T1 > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , { # [inline] fn to_superset (& self) -> UnitQuaternion < T2 > { let q = UnitQuaternion :: < T1 > :: from_rotation_matrix (self) ; q . to_superset () } # [inline] fn is_in_subset (q : & UnitQuaternion < T2 >) -> bool { crate :: is_convertible :: < _ , UnitQuaternion < T1 > > (q) } # [inline] fn from_superset_unchecked (q : & UnitQuaternion < T2 >) -> Self { let q : UnitQuaternion < T1 > = crate :: convert_ref_unchecked (q) ; q . to_rotation_matrix () } }
};
}
