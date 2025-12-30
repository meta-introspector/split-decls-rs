// Generated macro for impl_522 (impl)
macro_rules! Depcrate_element_basic_shapesimpl_522 {
() => {
// Module: crate::element::basic_shapes
// Provides: {"impl_522"}
// Dependencies: {}
impl < Coord , DB : DrawingBackend > Drawable < DB > for Pixel < Coord > { fn draw < I : Iterator < Item = BackendCoord > > (& self , mut points : I , backend : & mut DB , _ : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > { if let Some ((x , y)) = points . next () { return backend . draw_pixel ((x , y) , self . style . color . to_backend_color ()) ; } Ok (()) } }
};
}
