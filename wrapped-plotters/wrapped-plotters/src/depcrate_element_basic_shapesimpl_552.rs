// Generated macro for impl_552 (impl)
macro_rules! Depcrate_element_basic_shapesimpl_552 {
() => {
// Module: crate::element::basic_shapes
// Provides: {"impl_552"}
// Dependencies: {}
impl < 'a , Coord > PointCollection < 'a , Coord > for & 'a Polygon < Coord > { type Point = & 'a Coord ; type IntoIter = & 'a [Coord] ; fn point_iter (self) -> & 'a [Coord] { & self . points } }
};
}
