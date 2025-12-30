// Generated macro for impl_553 (impl)
macro_rules! Depcrate_element_basic_shapesimpl_553 {
() => {
// Module: crate::element::basic_shapes
// Provides: {"impl_553"}
// Dependencies: {}
impl < Coord , DB : DrawingBackend > Drawable < DB > for Polygon < Coord > { fn draw < I : Iterator < Item = BackendCoord > > (& self , points : I , backend : & mut DB , _ : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > { backend . fill_polygon (points , & self . style . color . to_backend_color ()) } }
};
}
