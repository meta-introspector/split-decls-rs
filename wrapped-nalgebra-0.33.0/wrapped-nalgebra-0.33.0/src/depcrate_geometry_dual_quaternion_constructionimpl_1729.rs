// Generated macro for impl_1729 (impl)
macro_rules! Depcrate_geometry_dual_quaternion_constructionimpl_1729 {
() => {
// Module: crate::geometry::dual_quaternion_construction
// Provides: {"impl_1729"}
// Dependencies: {}
impl < T : SimdRealField > UnitDualQuaternion < T > { # [doc = " The unit dual quaternion multiplicative identity, which also represents"] # [doc = " the identity transformation as an isometry."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::{UnitDualQuaternion, UnitQuaternion, Vector3, Point3};"] # [doc = " let ident = UnitDualQuaternion::identity();"] # [doc = " let point = Point3::new(1.0, -4.3, 3.33);"] # [doc = ""] # [doc = " assert_eq!(ident * point, point);"] # [doc = " assert_eq!(ident, ident.inverse());"] # [doc = " ```"] # [inline] pub fn identity () -> Self { Self :: new_unchecked (DualQuaternion :: identity ()) } # [doc = " Cast the components of `self` to another type."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::UnitDualQuaternion;"] # [doc = " let q = UnitDualQuaternion::<f64>::identity();"] # [doc = " let q2 = q.cast::<f32>();"] # [doc = " assert_eq!(q2, UnitDualQuaternion::<f32>::identity());"] # [doc = " ```"] pub fn cast < To : Scalar > (self) -> UnitDualQuaternion < To > where UnitDualQuaternion < To > : SupersetOf < Self > , { crate :: convert (self) } }
};
}
