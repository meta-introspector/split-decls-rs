// Generated macro for impl_2289 (impl)
macro_rules! Depcrate_geometry_isometry_conversionimpl_2289 {
() => {
// Module: crate::geometry::isometry_conversion
// Provides: {"impl_2289"}
// Dependencies: {}
impl < T1 , T2 , R1 , R2 , const D : usize > SubsetOf < Similarity < T2 , R2 , D > > for Isometry < T1 , R1 , D > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , R1 : AbstractRotation < T1 , D > + SubsetOf < R2 > , R2 : AbstractRotation < T2 , D > , { # [inline] fn to_superset (& self) -> Similarity < T2 , R2 , D > { Similarity :: from_isometry (self . to_superset () , T2 :: one ()) } # [inline] fn is_in_subset (sim : & Similarity < T2 , R2 , D >) -> bool { crate :: is_convertible :: < _ , Isometry < T1 , R1 , D > > (& sim . isometry) && sim . scaling () == T2 :: one () } # [inline] fn from_superset_unchecked (sim : & Similarity < T2 , R2 , D >) -> Self { crate :: convert_ref_unchecked (& sim . isometry) } }
};
}
