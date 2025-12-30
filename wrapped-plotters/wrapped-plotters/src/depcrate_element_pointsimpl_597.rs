// Generated macro for impl_597 (impl)
macro_rules! Depcrate_element_pointsimpl_597 {
() => {
// Module: crate::element::points
// Provides: {"impl_597"}
// Dependencies: {}
impl < Coord , DB : DrawingBackend , Size : SizeDesc > Drawable < DB > for TriangleMarker < Coord , Size > { fn draw < I : Iterator < Item = BackendCoord > > (& self , mut points : I , backend : & mut DB , ps : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > { if let Some ((x , y)) = points . next () { let size = self . size . in_pixels (& ps) ; let points = [- 90 , - 210 , - 330] . iter () . map (| deg | f64 :: from (* deg) * std :: f64 :: consts :: PI / 180.0) . map (| rad | { ((rad . cos () * f64 :: from (size) + f64 :: from (x)) . ceil () as i32 , (rad . sin () * f64 :: from (size) + f64 :: from (y)) . ceil () as i32 ,) }) ; backend . fill_polygon (points , & self . style . color . to_backend_color ()) ? ; } Ok (()) } }
};
}
