// Generated macro for impl_1554 (impl)
macro_rules! Depcrate_geometry_quaternion_constructionimpl_1554 {
() => {
// Module: crate::geometry::quaternion_construction
// Provides: {"impl_1554"}
// Dependencies: {}
impl < T > Quaternion < T > { # [doc = " Creates a quaternion from a 4D vector. The quaternion scalar part corresponds to the `w`"] # [doc = " vector component."] # [inline] pub const fn from_vector (vector : Vector4 < T >) -> Self { Self { coords : vector } } # [doc = " Creates a new quaternion from its individual components. Note that the arguments order does"] # [doc = " **not** follow the storage order."] # [doc = ""] # [doc = " The storage order is `[ i, j, k, w ]` while the arguments for this functions are in the"] # [doc = " order `(w, i, j, k)`."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::{Quaternion, Vector4};"] # [doc = " let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);"] # [doc = " assert!(q.i == 2.0 && q.j == 3.0 && q.k == 4.0 && q.w == 1.0);"] # [doc = " assert_eq!(*q.as_vector(), Vector4::new(2.0, 3.0, 4.0, 1.0));"] # [doc = " ```"] # [inline] pub const fn new (w : T , i : T , j : T , k : T) -> Self { Self :: from_vector (Vector4 :: new (i , j , k , w)) } # [doc = " Cast the components of `self` to another type."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::Quaternion;"] # [doc = " let q = Quaternion::new(1.0f64, 2.0, 3.0, 4.0);"] # [doc = " let q2 = q.cast::<f32>();"] # [doc = " assert_eq!(q2, Quaternion::new(1.0f32, 2.0, 3.0, 4.0));"] # [doc = " ```"] pub fn cast < To : Scalar > (self) -> Quaternion < To > where T : Scalar , To : SupersetOf < T > , { crate :: convert (self) } }
};
}
