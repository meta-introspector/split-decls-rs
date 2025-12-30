// Generated macro for impl_2542 (impl)
macro_rules! Depcrate_geometry_transform_constructionimpl_2542 {
() => {
// Module: crate::geometry::transform_construction
// Provides: {"impl_2542"}
// Dependencies: {}
impl < T : RealField , C : TCategory , const D : usize > Transform < T , C , D > where Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { # [doc = " Creates a new identity transform."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nalgebra::{Transform2, Projective2, Affine2, Transform3, Projective3, Affine3, Point2, Point3};"] # [doc = ""] # [doc = " let pt = Point2::new(1.0, 2.0);"] # [doc = " let t = Projective2::identity();"] # [doc = " assert_eq!(t * pt, pt);"] # [doc = ""] # [doc = " let aff = Affine2::identity();"] # [doc = " assert_eq!(aff * pt, pt);"] # [doc = ""] # [doc = " let aff = Transform2::identity();"] # [doc = " assert_eq!(aff * pt, pt);"] # [doc = ""] # [doc = " // Also works in 3D."] # [doc = " let pt = Point3::new(1.0, 2.0, 3.0);"] # [doc = " let t = Projective3::identity();"] # [doc = " assert_eq!(t * pt, pt);"] # [doc = ""] # [doc = " let aff = Affine3::identity();"] # [doc = " assert_eq!(aff * pt, pt);"] # [doc = ""] # [doc = " let aff = Transform3::identity();"] # [doc = " assert_eq!(aff * pt, pt);"] # [doc = " ```"] # [inline] pub fn identity () -> Self { Self :: from_matrix_unchecked (OMatrix :: < _ , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > , > :: identity ()) } }
};
}
