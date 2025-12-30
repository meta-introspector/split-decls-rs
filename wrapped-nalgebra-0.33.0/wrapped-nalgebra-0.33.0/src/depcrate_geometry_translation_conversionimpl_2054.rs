// Generated macro for impl_2054 (impl)
macro_rules! Depcrate_geometry_translation_conversionimpl_2054 {
() => {
// Module: crate::geometry::translation_conversion
// Provides: {"impl_2054"}
// Dependencies: {}
impl < T1 , T2 , const D : usize > SubsetOf < Translation < T2 , D > > for Translation < T1 , D > where T1 : Scalar , T2 : Scalar + SupersetOf < T1 > , { # [inline] fn to_superset (& self) -> Translation < T2 , D > { Translation :: from (self . vector . to_superset ()) } # [inline] fn is_in_subset (rot : & Translation < T2 , D >) -> bool { crate :: is_convertible :: < _ , SVector < T1 , D > > (& rot . vector) } # [inline] fn from_superset_unchecked (rot : & Translation < T2 , D >) -> Self { Translation { vector : rot . vector . to_subset_unchecked () , } } }
};
}
