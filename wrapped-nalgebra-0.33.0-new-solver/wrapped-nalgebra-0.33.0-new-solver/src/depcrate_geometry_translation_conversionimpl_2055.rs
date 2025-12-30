// Generated macro for impl_2055 (impl)
macro_rules! Depcrate_geometry_translation_conversionimpl_2055 {
() => {
// Module: crate::geometry::translation_conversion
// Provides: {"impl_2055"}
// Dependencies: {}
impl < T1 , T2 , R , const D : usize > SubsetOf < Isometry < T2 , R , D > > for Translation < T1 , D > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , R : AbstractRotation < T2 , D > , { # [inline] fn to_superset (& self) -> Isometry < T2 , R , D > { Isometry :: from_parts (self . to_superset () , R :: identity ()) } # [inline] fn is_in_subset (iso : & Isometry < T2 , R , D >) -> bool { iso . rotation == R :: identity () } # [inline] fn from_superset_unchecked (iso : & Isometry < T2 , R , D >) -> Self { Self :: from_superset_unchecked (& iso . translation) } }
};
}
