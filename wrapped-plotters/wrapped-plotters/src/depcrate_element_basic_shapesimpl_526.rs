// Generated macro for impl_526 (impl)
macro_rules! Depcrate_element_basic_shapesimpl_526 {
() => {
// Module: crate::element::basic_shapes
// Provides: {"impl_526"}
// Dependencies: {}
impl < Coord > PathElement < Coord > { # [doc = " Create a new path"] # [doc = " - `points`: The iterator of the points"] # [doc = " - `style`: The shape style"] # [doc = " - returns the created element"] pub fn new < P : Into < Vec < Coord > > , S : Into < ShapeStyle > > (points : P , style : S) -> Self { Self { points : points . into () , style : style . into () , } } }
};
}
