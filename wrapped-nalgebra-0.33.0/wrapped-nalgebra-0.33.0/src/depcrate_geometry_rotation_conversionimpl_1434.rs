// Generated macro for impl_1434 (impl)
macro_rules! Depcrate_geometry_rotation_conversionimpl_1434 {
() => {
// Module: crate::geometry::rotation_conversion
// Provides: {"impl_1434"}
// Dependencies: {}
impl < T1 , T2 > SubsetOf < UnitComplex < T2 > > for Rotation2 < T1 > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , { # [inline] fn to_superset (& self) -> UnitComplex < T2 > { let q = UnitComplex :: < T1 > :: from_rotation_matrix (self) ; q . to_superset () } # [inline] fn is_in_subset (q : & UnitComplex < T2 >) -> bool { crate :: is_convertible :: < _ , UnitComplex < T1 > > (q) } # [inline] fn from_superset_unchecked (q : & UnitComplex < T2 >) -> Self { let q : UnitComplex < T1 > = crate :: convert_ref_unchecked (q) ; q . to_rotation_matrix () } }
};
}
