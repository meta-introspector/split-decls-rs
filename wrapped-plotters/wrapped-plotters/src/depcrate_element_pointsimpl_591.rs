// Generated macro for impl_591 (impl)
macro_rules! Depcrate_element_pointsimpl_591 {
() => {
// Module: crate::element::points
// Provides: {"impl_591"}
// Dependencies: {}
impl < Coord , Size : SizeDesc > Cross < Coord , Size > { # [doc = "\n    Creates a cross marker.\n\n    See [`EmptyElement`] for more information and examples.\n    "] pub fn new < T : Into < ShapeStyle > > (coord : Coord , size : Size , style : T) -> Self { Self { center : coord , size , style : style . into () , } } }
};
}
