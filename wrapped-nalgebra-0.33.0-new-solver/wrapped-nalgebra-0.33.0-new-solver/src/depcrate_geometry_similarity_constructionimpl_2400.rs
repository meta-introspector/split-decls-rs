// Generated macro for impl_2400 (impl)
macro_rules! Depcrate_geometry_similarity_constructionimpl_2400 {
() => {
// Module: crate::geometry::similarity_construction
// Provides: {"impl_2400"}
// Dependencies: {}
impl < T : SimdRealField , R , const D : usize > Similarity < T , R , D > where T :: Element : SimdRealField , R : AbstractRotation < T , D > , { # [doc = " The similarity that applies the scaling factor `scaling`, followed by the rotation `r` with"] # [doc = " its axis passing through the point `p`."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # #[macro_use] extern crate approx;"] # [doc = " # use std::f32;"] # [doc = " # use nalgebra::{Similarity2, Point2, UnitComplex};"] # [doc = " let rot = UnitComplex::new(f32::consts::FRAC_PI_2);"] # [doc = " let pt = Point2::new(3.0, 2.0);"] # [doc = " let sim = Similarity2::rotation_wrt_point(rot, pt, 4.0);"] # [doc = ""] # [doc = " assert_relative_eq!(sim * Point2::new(1.0, 2.0), Point2::new(-3.0, 3.0), epsilon = 1.0e-6);"] # [doc = " ```"] # [inline] pub fn rotation_wrt_point (r : R , p : Point < T , D > , scaling : T) -> Self { let shift = r . transform_vector (& - & p . coords) ; Self :: from_parts (Translation :: from (shift + p . coords) , r , scaling) } }
};
}
