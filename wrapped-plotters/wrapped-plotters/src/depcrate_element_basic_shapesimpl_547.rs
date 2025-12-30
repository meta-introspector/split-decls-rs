// Generated macro for impl_547 (impl)
macro_rules! Depcrate_element_basic_shapesimpl_547 {
() => {
// Module: crate::element::basic_shapes
// Provides: {"impl_547"}
// Dependencies: {}
impl < 'a , Coord , Size : SizeDesc > PointCollection < 'a , Coord > for & 'a Circle < Coord , Size > { type Point = & 'a Coord ; type IntoIter = std :: iter :: Once < & 'a Coord > ; fn point_iter (self) -> std :: iter :: Once < & 'a Coord > { std :: iter :: once (& self . center) } }
};
}
