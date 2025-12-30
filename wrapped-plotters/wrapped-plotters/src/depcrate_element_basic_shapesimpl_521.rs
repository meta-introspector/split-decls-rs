// Generated macro for impl_521 (impl)
macro_rules! Depcrate_element_basic_shapesimpl_521 {
() => {
// Module: crate::element::basic_shapes
// Provides: {"impl_521"}
// Dependencies: {}
impl < 'a , Coord > PointCollection < 'a , Coord > for & 'a Pixel < Coord > { type Point = & 'a Coord ; type IntoIter = std :: iter :: Once < & 'a Coord > ; fn point_iter (self) -> Self :: IntoIter { std :: iter :: once (& self . pos) } }
};
}
