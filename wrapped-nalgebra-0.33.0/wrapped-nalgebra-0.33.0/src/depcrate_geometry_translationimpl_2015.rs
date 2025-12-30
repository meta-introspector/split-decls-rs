// Generated macro for impl_2015 (impl)
macro_rules! Depcrate_geometry_translationimpl_2015 {
() => {
// Module: crate::geometry::translation
// Provides: {"impl_2015"}
// Dependencies: {}
impl < T : Scalar + ClosedSubAssign , const D : usize > Translation < T , D > { # [doc = " Translate the given point by the inverse of this translation."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::{Translation3, Point3};"] # [doc = " let t = Translation3::new(1.0, 2.0, 3.0);"] # [doc = " let transformed_point = t.inverse_transform_point(&Point3::new(4.0, 5.0, 6.0));"] # [doc = " assert_eq!(transformed_point, Point3::new(3.0, 3.0, 3.0));"] # [doc = " ```"] # [inline] # [must_use] pub fn inverse_transform_point (& self , pt : & Point < T , D >) -> Point < T , D > { pt - & self . vector } }
};
}
