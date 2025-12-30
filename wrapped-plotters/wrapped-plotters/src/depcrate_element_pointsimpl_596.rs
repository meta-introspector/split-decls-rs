// Generated macro for impl_596 (impl)
macro_rules! Depcrate_element_pointsimpl_596 {
() => {
// Module: crate::element::points
// Provides: {"impl_596"}
// Dependencies: {}
impl < 'a , Coord : 'a , Size : SizeDesc > PointCollection < 'a , Coord > for & 'a TriangleMarker < Coord , Size > { type Point = & 'a Coord ; type IntoIter = std :: iter :: Once < & 'a Coord > ; fn point_iter (self) -> std :: iter :: Once < & 'a Coord > { std :: iter :: once (& self . center) } }
};
}
