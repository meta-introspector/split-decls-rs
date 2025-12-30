// Generated macro for impl_1741 (impl)
macro_rules! Depcrate_geometry_dual_quaternion_conversionimpl_1741 {
() => {
// Module: crate::geometry::dual_quaternion_conversion
// Provides: {"impl_1741"}
// Dependencies: {}
impl < T1 , T2 > SubsetOf < Similarity3 < T2 > > for UnitDualQuaternion < T1 > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , { # [inline] fn to_superset (& self) -> Similarity3 < T2 > { Similarity3 :: from_isometry (crate :: convert_ref (self) , T2 :: one ()) } # [inline] fn is_in_subset (sim : & Similarity3 < T2 >) -> bool { sim . scaling () == T2 :: one () } # [inline] fn from_superset_unchecked (sim : & Similarity3 < T2 >) -> Self { crate :: convert_ref_unchecked (& sim . isometry) } }
};
}
