// Generated macro for impl_1936 (impl)
macro_rules! Depcrate_geometry_unit_complex_conversionimpl_1936 {
() => {
// Module: crate::geometry::unit_complex_conversion
// Provides: {"impl_1936"}
// Dependencies: {}
impl < T1 , T2 , C > SubsetOf < Transform < T2 , C , 2 > > for UnitComplex < T1 > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , C : SuperTCategoryOf < TAffine > , { # [inline] fn to_superset (& self) -> Transform < T2 , C , 2 > { Transform :: from_matrix_unchecked (self . clone () . to_homogeneous () . to_superset ()) } # [inline] fn is_in_subset (t : & Transform < T2 , C , 2 >) -> bool { < Self as SubsetOf < _ > > :: is_in_subset (t . matrix ()) } # [inline] fn from_superset_unchecked (t : & Transform < T2 , C , 2 >) -> Self { Self :: from_superset_unchecked (t . matrix ()) } }
};
}
