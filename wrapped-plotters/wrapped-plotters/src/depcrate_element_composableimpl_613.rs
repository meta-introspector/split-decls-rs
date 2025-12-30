// Generated macro for impl_613 (impl)
macro_rules! Depcrate_element_composableimpl_613 {
() => {
// Module: crate::element::composable
// Provides: {"impl_613"}
// Dependencies: {}
impl < 'a , Coord , DB : DrawingBackend > PointCollection < 'a , Coord > for & 'a EmptyElement < Coord , DB > { type Point = & 'a Coord ; type IntoIter = Once < & 'a Coord > ; fn point_iter (self) -> Self :: IntoIter { once (& self . coord) } }
};
}
