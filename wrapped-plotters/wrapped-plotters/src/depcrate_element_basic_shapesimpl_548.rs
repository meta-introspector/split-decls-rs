// Generated macro for impl_548 (impl)
macro_rules! Depcrate_element_basic_shapesimpl_548 {
() => {
// Module: crate::element::basic_shapes
// Provides: {"impl_548"}
// Dependencies: {}
impl < Coord , DB : DrawingBackend , Size : SizeDesc > Drawable < DB > for Circle < Coord , Size > { fn draw < I : Iterator < Item = BackendCoord > > (& self , mut points : I , backend : & mut DB , ps : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > { if let Some ((x , y)) = points . next () { let size = self . size . in_pixels (& ps) . max (0) as u32 ; return backend . draw_circle ((x , y) , size , & self . style , self . style . filled) ; } Ok (()) } }
};
}
