// Generated macro for impl_562 (impl)
macro_rules! Depcrate_element_basic_shapes_3dimpl_562 {
() => {
// Module: crate::element::basic_shapes_3d
// Provides: {"impl_562"}
// Dependencies: {}
impl < 'a , X : 'a , Y : 'a , Z : 'a > PointCollection < 'a , (X , Y , Z) , BackendCoordAndZ > for & 'a Cubiod < X , Y , Z > { type Point = & 'a (X , Y , Z) ; type IntoIter = & 'a [(X , Y , Z)] ; fn point_iter (self) -> Self :: IntoIter { & self . vert } }
};
}
