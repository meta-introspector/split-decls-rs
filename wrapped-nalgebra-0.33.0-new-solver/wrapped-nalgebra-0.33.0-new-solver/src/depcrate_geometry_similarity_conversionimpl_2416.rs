// Generated macro for impl_2416 (impl)
macro_rules! Depcrate_geometry_similarity_conversionimpl_2416 {
() => {
// Module: crate::geometry::similarity_conversion
// Provides: {"impl_2416"}
// Dependencies: {}
impl < T1 , T2 , R1 , R2 , const D : usize > SubsetOf < Similarity < T2 , R2 , D > > for Similarity < T1 , R1 , D > where T1 : RealField + SubsetOf < T2 > , T2 : RealField + SupersetOf < T1 > , R1 : AbstractRotation < T1 , D > + SubsetOf < R2 > , R2 : AbstractRotation < T2 , D > , { # [inline] fn to_superset (& self) -> Similarity < T2 , R2 , D > { Similarity :: from_isometry (self . isometry . to_superset () , self . scaling () . to_superset ()) } # [inline] fn is_in_subset (sim : & Similarity < T2 , R2 , D >) -> bool { crate :: is_convertible :: < _ , Isometry < T1 , R1 , D > > (& sim . isometry) && crate :: is_convertible :: < _ , T1 > (& sim . scaling ()) } # [inline] fn from_superset_unchecked (sim : & Similarity < T2 , R2 , D >) -> Self { Similarity :: from_isometry (sim . isometry . to_subset_unchecked () , sim . scaling () . to_subset_unchecked () ,) } }
};
}
