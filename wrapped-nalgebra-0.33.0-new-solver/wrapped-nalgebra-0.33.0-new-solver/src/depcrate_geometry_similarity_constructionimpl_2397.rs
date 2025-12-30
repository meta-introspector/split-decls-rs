// Generated macro for impl_2397 (impl)
macro_rules! Depcrate_geometry_similarity_constructionimpl_2397 {
() => {
// Module: crate::geometry::similarity_construction
// Provides: {"impl_2397"}
// Dependencies: {}
impl < T : SimdRealField , R , const D : usize > Similarity < T , R , D > where T :: Element : SimdRealField , R : AbstractRotation < T , D > , { # [doc = " Creates a new identity similarity."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::{Similarity2, Point2, Similarity3, Point3};"] # [doc = ""] # [doc = " let sim = Similarity2::identity();"] # [doc = " let pt = Point2::new(1.0, 2.0);"] # [doc = " assert_eq!(sim * pt, pt);"] # [doc = ""] # [doc = " let sim = Similarity3::identity();"] # [doc = " let pt = Point3::new(1.0, 2.0, 3.0);"] # [doc = " assert_eq!(sim * pt, pt);"] # [doc = " ```"] # [inline] pub fn identity () -> Self { Self :: from_isometry (Isometry :: identity () , T :: one ()) } }
};
}
