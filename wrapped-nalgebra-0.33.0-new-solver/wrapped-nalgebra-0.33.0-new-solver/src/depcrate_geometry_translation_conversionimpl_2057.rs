// Generated macro for impl_2057 (impl)
macro_rules! Depcrate_geometry_translation_conversionimpl_2057 {
() => {
// Module: crate::geometry::translation_conversion
// Provides: {"impl_2057"}
// Dependencies: {}
impl < T1 , T2 , R , const D : usize > SubsetOf < Similarity < T2 , R , D > > for Translation < T1 , D > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , R : AbstractRotation < T2 , D > , { # [inline] fn to_superset (& self) -> Similarity < T2 , R , D > { Similarity :: from_parts (self . to_superset () , R :: identity () , T2 :: one ()) } # [inline] fn is_in_subset (sim : & Similarity < T2 , R , D >) -> bool { sim . isometry . rotation == R :: identity () && sim . scaling () == T2 :: one () } # [inline] fn from_superset_unchecked (sim : & Similarity < T2 , R , D >) -> Self { Self :: from_superset_unchecked (& sim . isometry . translation) } }
};
}
