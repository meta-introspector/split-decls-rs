// Generated macro for impl_1932 (impl)
macro_rules! Depcrate_geometry_unit_complex_conversionimpl_1932 {
() => {
// Module: crate::geometry::unit_complex_conversion
// Provides: {"impl_1932"}
// Dependencies: {}
impl < T1 , T2 > SubsetOf < UnitComplex < T2 > > for UnitComplex < T1 > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , { # [inline] fn to_superset (& self) -> UnitComplex < T2 > { UnitComplex :: new_unchecked (self . as_ref () . to_superset ()) } # [inline] fn is_in_subset (uq : & UnitComplex < T2 >) -> bool { crate :: is_convertible :: < _ , Complex < T1 > > (uq . as_ref ()) } # [inline] fn from_superset_unchecked (uq : & UnitComplex < T2 >) -> Self { Self :: new_unchecked (crate :: convert_ref_unchecked (uq . as_ref ())) } }
};
}
