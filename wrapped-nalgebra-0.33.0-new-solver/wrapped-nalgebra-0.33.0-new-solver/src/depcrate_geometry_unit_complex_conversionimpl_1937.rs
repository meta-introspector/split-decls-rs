// Generated macro for impl_1937 (impl)
macro_rules! Depcrate_geometry_unit_complex_conversionimpl_1937 {
() => {
// Module: crate::geometry::unit_complex_conversion
// Provides: {"impl_1937"}
// Dependencies: {}
impl < T1 : RealField , T2 : RealField + SupersetOf < T1 > > SubsetOf < Matrix3 < T2 > > for UnitComplex < T1 > { # [inline] fn to_superset (& self) -> Matrix3 < T2 > { self . clone () . to_homogeneous () . to_superset () } # [inline] fn is_in_subset (m : & Matrix3 < T2 >) -> bool { crate :: is_convertible :: < _ , Rotation2 < T1 > > (m) } # [inline] fn from_superset_unchecked (m : & Matrix3 < T2 >) -> Self { let rot : Rotation2 < T1 > = crate :: convert_ref_unchecked (m) ; Self :: from_rotation_matrix (& rot) } }
};
}
