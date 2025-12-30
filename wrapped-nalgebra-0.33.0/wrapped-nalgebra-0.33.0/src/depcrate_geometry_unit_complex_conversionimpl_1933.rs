// Generated macro for impl_1933 (impl)
macro_rules! Depcrate_geometry_unit_complex_conversionimpl_1933 {
() => {
// Module: crate::geometry::unit_complex_conversion
// Provides: {"impl_1933"}
// Dependencies: {}
impl < T1 , T2 > SubsetOf < Rotation2 < T2 > > for UnitComplex < T1 > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , { # [inline] fn to_superset (& self) -> Rotation2 < T2 > { let q : UnitComplex < T2 > = self . to_superset () ; q . to_rotation_matrix () . to_superset () } # [inline] fn is_in_subset (rot : & Rotation2 < T2 >) -> bool { crate :: is_convertible :: < _ , Rotation2 < T1 > > (rot) } # [inline] fn from_superset_unchecked (rot : & Rotation2 < T2 >) -> Self { let q = UnitComplex :: < T2 > :: from_rotation_matrix (rot) ; crate :: convert_unchecked (q) } }
};
}
