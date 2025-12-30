// Generated macro for impl_620 (impl)
macro_rules! Depcrate_element_composableimpl_620 {
() => {
// Module: crate::element::composable
// Provides: {"impl_620"}
// Dependencies: {}
impl < 'b , Coord , DB : DrawingBackend , A , B > PointCollection < 'b , Coord > for & 'b ComposedElement < Coord , DB , A , B > where A : Drawable < DB > , B : Drawable < DB > , { type Point = & 'b Coord ; type IntoIter = Once < & 'b Coord > ; fn point_iter (self) -> Self :: IntoIter { once (& self . offset) } }
};
}
