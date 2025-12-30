// Generated macro for impl_593 (impl)
macro_rules! Depcrate_element_pointsimpl_593 {
() => {
// Module: crate::element::points
// Provides: {"impl_593"}
// Dependencies: {}
impl < Coord , DB : DrawingBackend , Size : SizeDesc > Drawable < DB > for Cross < Coord , Size > { fn draw < I : Iterator < Item = BackendCoord > > (& self , mut points : I , backend : & mut DB , ps : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > { if let Some ((x , y)) = points . next () { let size = self . size . in_pixels (& ps) ; let (x0 , y0) = (x - size , y - size) ; let (x1 , y1) = (x + size , y + size) ; backend . draw_line ((x0 , y0) , (x1 , y1) , & self . style) ? ; backend . draw_line ((x0 , y1) , (x1 , y0) , & self . style) ? ; } Ok (()) } }
};
}
