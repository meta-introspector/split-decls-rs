// Generated macro for impl_2156 (impl)
macro_rules! Depcrate_geometry_scale_constructionimpl_2156 {
() => {
// Module: crate::geometry::scale_construction
// Provides: {"impl_2156"}
// Dependencies: {}
impl < T : Scalar , const D : usize > Scale < T , D > { # [doc = " Creates a new identity scale."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::{Point2, Point3, Scale2, Scale3};"] # [doc = " let t = Scale2::identity();"] # [doc = " let p = Point2::new(1.0, 2.0);"] # [doc = " assert_eq!(t * p, p);"] # [doc = ""] # [doc = " // Works in all dimensions."] # [doc = " let t = Scale3::identity();"] # [doc = " let p = Point3::new(1.0, 2.0, 3.0);"] # [doc = " assert_eq!(t * p, p);"] # [doc = " ```"] # [inline] pub fn identity () -> Scale < T , D > where T : One , { Scale :: from (SVector :: from_element (T :: one ())) } # [doc = " Cast the components of `self` to another type."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::Scale2;"] # [doc = " let tra = Scale2::new(1.0f64, 2.0);"] # [doc = " let tra2 = tra.cast::<f32>();"] # [doc = " assert_eq!(tra2, Scale2::new(1.0f32, 2.0));"] # [doc = " ```"] pub fn cast < To : Scalar > (self) -> Scale < To , D > where Scale < To , D > : SupersetOf < Self > , { crate :: convert (self) } }
};
}
