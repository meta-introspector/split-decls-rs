// Generated macro for impl_592 (impl)
macro_rules! Depcrate_element_pointsimpl_592 {
() => {
// Module: crate::element::points
// Provides: {"impl_592"}
// Dependencies: {}
impl < 'a , Coord : 'a , Size : SizeDesc > PointCollection < 'a , Coord > for & 'a Cross < Coord , Size > { type Point = & 'a Coord ; type IntoIter = std :: iter :: Once < & 'a Coord > ; fn point_iter (self) -> std :: iter :: Once < & 'a Coord > { std :: iter :: once (& self . center) } }
};
}
