// Generated macro for impl_528 (impl)
macro_rules! Depcrate_element_basic_shapesimpl_528 {
() => {
// Module: crate::element::basic_shapes
// Provides: {"impl_528"}
// Dependencies: {}
impl < Coord , DB : DrawingBackend > Drawable < DB > for PathElement < Coord > { fn draw < I : Iterator < Item = BackendCoord > > (& self , points : I , backend : & mut DB , _ : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > { backend . draw_path (points , & self . style) } }
};
}
