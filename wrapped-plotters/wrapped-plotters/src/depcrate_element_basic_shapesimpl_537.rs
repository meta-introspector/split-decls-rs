// Generated macro for impl_537 (impl)
macro_rules! Depcrate_element_basic_shapesimpl_537 {
() => {
// Module: crate::element::basic_shapes
// Provides: {"impl_537"}
// Dependencies: {}
impl < 'a , I : Iterator + Clone , Size : SizeDesc , Marker > PointCollection < 'a , I :: Item > for & 'a DottedPathElement < I , Size , Marker > { type Point = I :: Item ; type IntoIter = I ; fn point_iter (self) -> Self :: IntoIter { self . points . clone () } }
};
}
