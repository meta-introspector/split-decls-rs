// Generated macro for impl_698 (impl)
macro_rules! Depcrate_element_dynelemimpl_698 {
() => {
// Module: crate::element::dynelem
// Provides: {"impl_698"}
// Dependencies: {}
impl < 'a , 'b : 'a , DB : DrawingBackend , Coord : Clone > PointCollection < 'a , Coord > for & 'a DynElement < 'b , DB , Coord > { type Point = & 'a Coord ; type IntoIter = & 'a Vec < Coord > ; fn point_iter (self) -> Self :: IntoIter { & self . points } }
};
}
