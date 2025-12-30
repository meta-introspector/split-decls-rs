// Generated macro for impl_616 (impl)
macro_rules! Depcrate_element_composableimpl_616 {
() => {
// Module: crate::element::composable
// Provides: {"impl_616"}
// Dependencies: {}
impl < 'b , Coord , DB : DrawingBackend , A : Drawable < DB > > PointCollection < 'b , Coord > for & 'b BoxedElement < Coord , DB , A > { type Point = & 'b Coord ; type IntoIter = Once < & 'b Coord > ; fn point_iter (self) -> Self :: IntoIter { once (& self . offset) } }
};
}
