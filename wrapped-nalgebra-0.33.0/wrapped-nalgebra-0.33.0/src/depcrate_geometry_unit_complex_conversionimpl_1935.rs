// Generated macro for impl_1935 (impl)
macro_rules! Depcrate_geometry_unit_complex_conversionimpl_1935 {
() => {
// Module: crate::geometry::unit_complex_conversion
// Provides: {"impl_1935"}
// Dependencies: {}
impl < T1 , T2 , R > SubsetOf < Similarity < T2 , R , 2 > > for UnitComplex < T1 > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , R : AbstractRotation < T2 , 2 > + SupersetOf < Self > , { # [inline] fn to_superset (& self) -> Similarity < T2 , R , 2 > { Similarity :: from_isometry (crate :: convert_ref (self) , T2 :: one ()) } # [inline] fn is_in_subset (sim : & Similarity < T2 , R , 2 >) -> bool { sim . isometry . translation . vector . is_zero () && sim . scaling () == T2 :: one () } # [inline] fn from_superset_unchecked (sim : & Similarity < T2 , R , 2 >) -> Self { crate :: convert_ref_unchecked (& sim . isometry) } }
};
}
