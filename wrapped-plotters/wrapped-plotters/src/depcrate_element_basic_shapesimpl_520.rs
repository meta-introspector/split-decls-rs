// Generated macro for impl_520 (impl)
macro_rules! Depcrate_element_basic_shapesimpl_520 {
() => {
// Module: crate::element::basic_shapes
// Provides: {"impl_520"}
// Dependencies: {}
impl < Coord > Pixel < Coord > { # [doc = "\n    Creates a new pixel.\n\n    See [`crate::element::EmptyElement`] for more information and examples.\n    "] pub fn new < P : Into < Coord > , S : Into < ShapeStyle > > (pos : P , style : S) -> Self { Self { pos : pos . into () , style : style . into () , } } }
};
}
