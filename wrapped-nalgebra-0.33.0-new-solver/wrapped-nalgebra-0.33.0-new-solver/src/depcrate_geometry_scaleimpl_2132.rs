// Generated macro for impl_2132 (impl)
macro_rules! Depcrate_geometry_scaleimpl_2132 {
() => {
// Module: crate::geometry::scale
// Provides: {"impl_2132"}
// Dependencies: {}
impl < T : Scalar + ClosedMulAssign , const D : usize > Scale < T , D > { # [doc = " Translate the given point."] # [doc = ""] # [doc = " This is the same as the multiplication `self * pt`."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::{Scale3, Point3};"] # [doc = " let t = Scale3::new(1.0, 2.0, 3.0);"] # [doc = " let transformed_point = t.transform_point(&Point3::new(4.0, 5.0, 6.0));"] # [doc = " assert_eq!(transformed_point, Point3::new(4.0, 10.0, 18.0));"] # [doc = " ```"] # [inline] # [must_use] pub fn transform_point (& self , pt : & Point < T , D >) -> Point < T , D > { self * pt } }
};
}
