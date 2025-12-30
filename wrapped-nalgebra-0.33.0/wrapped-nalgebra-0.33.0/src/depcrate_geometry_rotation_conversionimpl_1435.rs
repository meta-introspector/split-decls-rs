// Generated macro for impl_1435 (impl)
macro_rules! Depcrate_geometry_rotation_conversionimpl_1435 {
() => {
// Module: crate::geometry::rotation_conversion
// Provides: {"impl_1435"}
// Dependencies: {}
impl < T1 , T2 , R , const D : usize > SubsetOf < Isometry < T2 , R , D > > for Rotation < T1 , D > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , R : AbstractRotation < T2 , D > + SupersetOf < Self > , { # [inline] fn to_superset (& self) -> Isometry < T2 , R , D > { Isometry :: from_parts (Translation :: identity () , crate :: convert_ref (self)) } # [inline] fn is_in_subset (iso : & Isometry < T2 , R , D >) -> bool { iso . translation . vector . is_zero () } # [inline] fn from_superset_unchecked (iso : & Isometry < T2 , R , D >) -> Self { crate :: convert_ref_unchecked (& iso . rotation) } }
};
}
