// Generated macro for impl_1436 (impl)
macro_rules! Depcrate_geometry_rotation_conversionimpl_1436 {
() => {
// Module: crate::geometry::rotation_conversion
// Provides: {"impl_1436"}
// Dependencies: {}
impl < T1 , T2 , R , const D : usize > SubsetOf < Similarity < T2 , R , D > > for Rotation < T1 , D > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , R : AbstractRotation < T2 , D > + SupersetOf < Self > , { # [inline] fn to_superset (& self) -> Similarity < T2 , R , D > { Similarity :: from_parts (Translation :: identity () , crate :: convert_ref (self) , T2 :: one ()) } # [inline] fn is_in_subset (sim : & Similarity < T2 , R , D >) -> bool { sim . isometry . translation . vector . is_zero () && sim . scaling () == T2 :: one () } # [inline] fn from_superset_unchecked (sim : & Similarity < T2 , R , D >) -> Self { crate :: convert_ref_unchecked (& sim . isometry . rotation) } }
};
}
