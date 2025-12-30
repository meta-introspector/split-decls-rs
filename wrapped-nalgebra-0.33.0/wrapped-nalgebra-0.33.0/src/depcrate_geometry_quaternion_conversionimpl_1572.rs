// Generated macro for impl_1572 (impl)
macro_rules! Depcrate_geometry_quaternion_conversionimpl_1572 {
() => {
// Module: crate::geometry::quaternion_conversion
// Provides: {"impl_1572"}
// Dependencies: {}
impl < T1 , T2 > SubsetOf < Quaternion < T2 > > for Quaternion < T1 > where T1 : Scalar , T2 : Scalar + SupersetOf < T1 > , { # [inline] fn to_superset (& self) -> Quaternion < T2 > { Quaternion :: from (self . coords . to_superset ()) } # [inline] fn is_in_subset (q : & Quaternion < T2 >) -> bool { crate :: is_convertible :: < _ , Vector4 < T1 > > (& q . coords) } # [inline] fn from_superset_unchecked (q : & Quaternion < T2 >) -> Self { Self { coords : q . coords . to_subset_unchecked () , } } }
};
}
