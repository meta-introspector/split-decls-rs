// Generated macro for impl_532 (impl)
macro_rules! Depcrate_element_basic_shapesimpl_532 {
() => {
// Module: crate::element::basic_shapes
// Provides: {"impl_532"}
// Dependencies: {}
impl < 'a , I : Iterator + Clone , Size : SizeDesc > PointCollection < 'a , I :: Item > for & 'a DashedPathElement < I , Size > { type Point = I :: Item ; type IntoIter = I ; fn point_iter (self) -> Self :: IntoIter { self . points . clone () } }
};
}
