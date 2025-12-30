// Generated macro for impl_1575 (impl)
macro_rules! Depcrate_geometry_quaternion_conversionimpl_1575 {
() => {
// Module: crate::geometry::quaternion_conversion
// Provides: {"impl_1575"}
// Dependencies: {}
impl < T1 , T2 , R > SubsetOf < Isometry < T2 , R , 3 > > for UnitQuaternion < T1 > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , R : AbstractRotation < T2 , 3 > + SupersetOf < Self > , { # [inline] fn to_superset (& self) -> Isometry < T2 , R , 3 > { Isometry :: from_parts (Translation :: identity () , crate :: convert_ref (self)) } # [inline] fn is_in_subset (iso : & Isometry < T2 , R , 3 >) -> bool { iso . translation . vector . is_zero () } # [inline] fn from_superset_unchecked (iso : & Isometry < T2 , R , 3 >) -> Self { crate :: convert_ref_unchecked (& iso . rotation) } }
};
}
