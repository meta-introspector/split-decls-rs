// Generated macro for impl_1556 (impl)
macro_rules! Depcrate_geometry_quaternion_constructionimpl_1556 {
() => {
// Module: crate::geometry::quaternion_construction
// Provides: {"impl_1556"}
// Dependencies: {}
impl < T : SimdRealField > Quaternion < T > where T :: Element : SimdRealField , { # [doc = " Creates a new quaternion from its polar decomposition."] # [doc = ""] # [doc = " Note that `axis` is assumed to be a unit vector."] pub fn from_polar_decomposition < SB > (scale : T , theta : T , axis : Unit < Vector < T , U3 , SB > >) -> Self where SB : Storage < T , U3 > , { let rot = UnitQuaternion :: < T > :: from_axis_angle (& axis , theta * crate :: convert (2.0f64)) ; rot . into_inner () * scale } }
};
}
