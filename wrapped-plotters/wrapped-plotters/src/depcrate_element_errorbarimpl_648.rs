// Generated macro for impl_648 (impl)
macro_rules! Depcrate_element_errorbarimpl_648 {
() => {
// Module: crate::element::errorbar
// Provides: {"impl_648"}
// Dependencies: {}
impl < K , V , O : ErrorBarOrient < K , V > , DB : DrawingBackend > Drawable < DB > for ErrorBar < K , V , O > { fn draw < I : Iterator < Item = BackendCoord > > (& self , points : I , backend : & mut DB , _ : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > { let points : Vec < _ > = points . take (3) . collect () ; let (from , to) = O :: ending_coord (points [0] , self . width) ; backend . draw_line (from , to , & self . style) ? ; let (from , to) = O :: ending_coord (points [2] , self . width) ; backend . draw_line (from , to , & self . style) ? ; backend . draw_line (points [0] , points [2] , & self . style) ? ; backend . draw_circle (points [1] , self . width / 2 , & self . style , self . style . filled) ? ; Ok (()) } }
};
}
