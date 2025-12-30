// Generated macro for impl_611 (impl)
macro_rules! Depcrate_element_composableimpl_611 {
() => {
// Module: crate::element::composable
// Provides: {"impl_611"}
// Dependencies: {}
impl < Coord , DB : DrawingBackend > EmptyElement < Coord , DB > { # [doc = "\n    An empty composable element. This is the starting point of a composed element.\n\n    See [`EmptyElement`] for more information and examples.\n    "] pub fn at (coord : Coord) -> Self { Self { coord , phantom : PhantomData , } } }
};
}
