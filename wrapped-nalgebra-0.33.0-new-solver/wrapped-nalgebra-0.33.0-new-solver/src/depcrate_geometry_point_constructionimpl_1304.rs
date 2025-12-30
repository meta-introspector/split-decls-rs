// Generated macro for impl_1304 (impl)
macro_rules! Depcrate_geometry_point_constructionimpl_1304 {
() => {
// Module: crate::geometry::point_construction
// Provides: {"impl_1304"}
// Dependencies: {}
# [doc = " # Construction from individual components"] impl < T : Scalar > Point1 < T > { # [doc = " Initializes this point from its components."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nalgebra::Point1;"] # [doc = " let p = Point1::new(1.0);"] # [doc = " assert_eq!(p.x, 1.0);"] # [doc = " ```"] # [inline] pub const fn new (x : T) -> Self { Point { coords : Vector1 :: new (x) , } } }
};
}
