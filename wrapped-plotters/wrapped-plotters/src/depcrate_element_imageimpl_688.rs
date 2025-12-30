// Generated macro for impl_688 (impl)
macro_rules! Depcrate_element_imageimpl_688 {
() => {
// Module: crate::element::image
// Provides: {"impl_688"}
// Dependencies: {}
impl < 'a , 'b , Coord > PointCollection < 'a , Coord > for & 'a BitMapElement < 'b , Coord > { type Point = & 'a Coord ; type IntoIter = std :: iter :: Once < & 'a Coord > ; fn point_iter (self) -> Self :: IntoIter { std :: iter :: once (& self . pos) } }
};
}
