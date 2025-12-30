// Generated macro for impl_595 (impl)
macro_rules! Depcrate_element_pointsimpl_595 {
() => {
// Module: crate::element::points
// Provides: {"impl_595"}
// Dependencies: {}
impl < Coord , Size : SizeDesc > TriangleMarker < Coord , Size > { # [doc = "\n    Creates a triangle marker.\n\n    See [`EmptyElement`] for more information and examples.\n    "] pub fn new < T : Into < ShapeStyle > > (coord : Coord , size : Size , style : T) -> Self { Self { center : coord , size , style : style . into () , } } }
};
}
