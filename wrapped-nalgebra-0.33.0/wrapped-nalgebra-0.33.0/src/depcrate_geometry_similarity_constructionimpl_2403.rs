// Generated macro for impl_2403 (impl)
macro_rules! Depcrate_geometry_similarity_constructionimpl_2403 {
() => {
// Module: crate::geometry::similarity_construction
// Provides: {"impl_2403"}
// Dependencies: {}
impl < T : SimdRealField > Similarity < T , UnitComplex < T > , 2 > where T :: Element : SimdRealField , { # [doc = " Creates a new similarity from a translation and a rotation angle."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # #[macro_use] extern crate approx;"] # [doc = " # use std::f32;"] # [doc = " # use nalgebra::{Similarity2, Vector2, Point2};"] # [doc = " let sim = Similarity2::new(Vector2::new(1.0, 2.0), f32::consts::FRAC_PI_2, 3.0);"] # [doc = ""] # [doc = " assert_relative_eq!(sim * Point2::new(2.0, 4.0), Point2::new(-11.0, 8.0), epsilon = 1.0e-6);"] # [doc = " ```"] # [inline] pub fn new (translation : Vector2 < T > , angle : T , scaling : T) -> Self { Self :: from_parts (Translation :: from (translation) , UnitComplex :: new (angle) , scaling ,) } # [doc = " Cast the components of `self` to another type."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::Similarity2;"] # [doc = " let sim = Similarity2::<f64>::identity();"] # [doc = " let sim2 = sim.cast::<f32>();"] # [doc = " assert_eq!(sim2, Similarity2::<f32>::identity());"] # [doc = " ```"] pub fn cast < To : Scalar > (self) -> Similarity < To , UnitComplex < To > , 2 > where Similarity < To , UnitComplex < To > , 2 > : SupersetOf < Self > , { crate :: convert (self) } }
};
}
