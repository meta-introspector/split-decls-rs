// Generated macro for impl_1577 (impl)
macro_rules! Depcrate_geometry_quaternion_conversionimpl_1577 {
() => {
// Module: crate::geometry::quaternion_conversion
// Provides: {"impl_1577"}
// Dependencies: {}
impl < T1 , T2 , R > SubsetOf < Similarity < T2 , R , 3 > > for UnitQuaternion < T1 > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , R : AbstractRotation < T2 , 3 > + SupersetOf < Self > , { # [inline] fn to_superset (& self) -> Similarity < T2 , R , 3 > { Similarity :: from_isometry (crate :: convert_ref (self) , T2 :: one ()) } # [inline] fn is_in_subset (sim : & Similarity < T2 , R , 3 >) -> bool { sim . isometry . translation . vector . is_zero () && sim . scaling () == T2 :: one () } # [inline] fn from_superset_unchecked (sim : & Similarity < T2 , R , 3 >) -> Self { crate :: convert_ref_unchecked (& sim . isometry) } }
};
}
