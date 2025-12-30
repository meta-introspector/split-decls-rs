// Generated macro for impl_2133 (impl)
macro_rules! Depcrate_geometry_scaleimpl_2133 {
() => {
// Module: crate::geometry::scale
// Provides: {"impl_2133"}
// Dependencies: {}
impl < T : Scalar + ClosedDivAssign + ClosedMulAssign + One + Zero , const D : usize > Scale < T , D > { # [doc = " Translate the given point by the inverse of this Scale."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::{Scale3, Point3};"] # [doc = " let t = Scale3::new(1.0, 2.0, 3.0);"] # [doc = " let transformed_point = t.try_inverse_transform_point(&Point3::new(4.0, 6.0, 6.0)).unwrap();"] # [doc = " assert_eq!(transformed_point, Point3::new(4.0, 3.0, 2.0));"] # [doc = ""] # [doc = " // Returns None if the inverse doesn't exist."] # [doc = " let t = Scale3::new(1.0, 0.0, 3.0);"] # [doc = " let transformed_point = t.try_inverse_transform_point(&Point3::new(4.0, 6.0, 6.0));"] # [doc = " assert_eq!(transformed_point, None);"] # [doc = " ```"] # [inline] # [must_use] pub fn try_inverse_transform_point (& self , pt : & Point < T , D >) -> Option < Point < T , D > > { self . try_inverse () . map (| s | s * pt) } }
};
}
