// Generated macro for impl_1934 (impl)
macro_rules! Depcrate_geometry_unit_complex_conversionimpl_1934 {
() => {
// Module: crate::geometry::unit_complex_conversion
// Provides: {"impl_1934"}
// Dependencies: {}
impl < T1 , T2 , R > SubsetOf < Isometry < T2 , R , 2 > > for UnitComplex < T1 > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , R : AbstractRotation < T2 , 2 > + SupersetOf < Self > , { # [inline] fn to_superset (& self) -> Isometry < T2 , R , 2 > { Isometry :: from_parts (Translation :: identity () , crate :: convert_ref (self)) } # [inline] fn is_in_subset (iso : & Isometry < T2 , R , 2 >) -> bool { iso . translation . vector . is_zero () } # [inline] fn from_superset_unchecked (iso : & Isometry < T2 , R , 2 >) -> Self { crate :: convert_ref_unchecked (& iso . rotation) } }
};
}
