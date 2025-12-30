// Generated macro for impl_542 (impl)
macro_rules! Depcrate_element_basic_shapesimpl_542 {
() => {
// Module: crate::element::basic_shapes
// Provides: {"impl_542"}
// Dependencies: {}
impl < 'a , Coord > PointCollection < 'a , Coord > for & 'a Rectangle < Coord > { type Point = & 'a Coord ; type IntoIter = & 'a [Coord] ; fn point_iter (self) -> & 'a [Coord] { & self . points } }
};
}
