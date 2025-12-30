// Generated macro for impl_527 (impl)
macro_rules! Depcrate_element_basic_shapesimpl_527 {
() => {
// Module: crate::element::basic_shapes
// Provides: {"impl_527"}
// Dependencies: {}
impl < 'a , Coord > PointCollection < 'a , Coord > for & 'a PathElement < Coord > { type Point = & 'a Coord ; type IntoIter = & 'a [Coord] ; fn point_iter (self) -> & 'a [Coord] { & self . points } }
};
}
