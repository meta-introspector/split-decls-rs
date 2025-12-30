// Generated macro for impl_2171 (impl)
macro_rules! Depcrate_geometry_scale_conversionimpl_2171 {
() => {
// Module: crate::geometry::scale_conversion
// Provides: {"impl_2171"}
// Dependencies: {}
impl < T1 , T2 , const D : usize > SubsetOf < Scale < T2 , D > > for Scale < T1 , D > where T1 : Scalar , T2 : Scalar + SupersetOf < T1 > , { # [inline] fn to_superset (& self) -> Scale < T2 , D > { Scale :: from (self . vector . to_superset ()) } # [inline] fn is_in_subset (rot : & Scale < T2 , D >) -> bool { crate :: is_convertible :: < _ , SVector < T1 , D > > (& rot . vector) } # [inline] fn from_superset_unchecked (rot : & Scale < T2 , D >) -> Self { Scale { vector : rot . vector . to_subset_unchecked () , } } }
};
}
