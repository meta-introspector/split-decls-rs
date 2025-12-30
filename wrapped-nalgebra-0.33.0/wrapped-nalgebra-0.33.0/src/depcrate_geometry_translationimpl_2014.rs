// Generated macro for impl_2014 (impl)
macro_rules! Depcrate_geometry_translationimpl_2014 {
() => {
// Module: crate::geometry::translation
// Provides: {"impl_2014"}
// Dependencies: {}
impl < T : Scalar + ClosedAddAssign , const D : usize > Translation < T , D > { # [doc = " Translate the given point."] # [doc = ""] # [doc = " This is the same as the multiplication `self * pt`."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::{Translation3, Point3};"] # [doc = " let t = Translation3::new(1.0, 2.0, 3.0);"] # [doc = " let transformed_point = t.transform_point(&Point3::new(4.0, 5.0, 6.0));"] # [doc = " assert_eq!(transformed_point, Point3::new(5.0, 7.0, 9.0));"] # [doc = " ```"] # [inline] # [must_use] pub fn transform_point (& self , pt : & Point < T , D >) -> Point < T , D > { pt + & self . vector } }
};
}
