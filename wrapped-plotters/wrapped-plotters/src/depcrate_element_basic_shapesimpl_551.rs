// Generated macro for impl_551 (impl)
macro_rules! Depcrate_element_basic_shapesimpl_551 {
() => {
// Module: crate::element::basic_shapes
// Provides: {"impl_551"}
// Dependencies: {}
impl < Coord > Polygon < Coord > { # [doc = " Create a new polygon"] # [doc = " - `points`: The iterator of the points"] # [doc = " - `style`: The shape style"] # [doc = " - returns the created element"] pub fn new < P : Into < Vec < Coord > > , S : Into < ShapeStyle > > (points : P , style : S) -> Self { Self { points : points . into () , style : style . into () , } } }
};
}
