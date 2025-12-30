// Generated macro for impl_2039 (impl)
macro_rules! Depcrate_geometry_translation_constructionimpl_2039 {
() => {
// Module: crate::geometry::translation_construction
// Provides: {"impl_2039"}
// Dependencies: {}
impl < T : Scalar , const D : usize > Translation < T , D > { # [doc = " Creates a new identity translation."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::{Point2, Point3, Translation2, Translation3};"] # [doc = " let t = Translation2::identity();"] # [doc = " let p = Point2::new(1.0, 2.0);"] # [doc = " assert_eq!(t * p, p);"] # [doc = ""] # [doc = " // Works in all dimensions."] # [doc = " let t = Translation3::identity();"] # [doc = " let p = Point3::new(1.0, 2.0, 3.0);"] # [doc = " assert_eq!(t * p, p);"] # [doc = " ```"] # [inline] pub fn identity () -> Translation < T , D > where T : Zero , { Self :: from (SVector :: < T , D > :: from_element (T :: zero ())) } # [doc = " Cast the components of `self` to another type."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::Translation2;"] # [doc = " let tra = Translation2::new(1.0f64, 2.0);"] # [doc = " let tra2 = tra.cast::<f32>();"] # [doc = " assert_eq!(tra2, Translation2::new(1.0f32, 2.0));"] # [doc = " ```"] pub fn cast < To : Scalar > (self) -> Translation < To , D > where Translation < To , D > : SupersetOf < Self > , { crate :: convert (self) } }
};
}
