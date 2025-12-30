// Generated macro for impl_1742 (impl)
macro_rules! Depcrate_geometry_dual_quaternion_conversionimpl_1742 {
() => {
// Module: crate::geometry::dual_quaternion_conversion
// Provides: {"impl_1742"}
// Dependencies: {}
impl < T1 , T2 , C > SubsetOf < Transform < T2 , C , 3 > > for UnitDualQuaternion < T1 > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , C : SuperTCategoryOf < TAffine > , { # [inline] fn to_superset (& self) -> Transform < T2 , C , 3 > { Transform :: from_matrix_unchecked (self . clone () . to_homogeneous () . to_superset ()) } # [inline] fn is_in_subset (t : & Transform < T2 , C , 3 >) -> bool { < Self as SubsetOf < _ > > :: is_in_subset (t . matrix ()) } # [inline] fn from_superset_unchecked (t : & Transform < T2 , C , 3 >) -> Self { Self :: from_superset_unchecked (t . matrix ()) } }
};
}
