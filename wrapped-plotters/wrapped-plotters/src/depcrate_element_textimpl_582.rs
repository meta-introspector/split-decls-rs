// Generated macro for impl_582 (impl)
macro_rules! Depcrate_element_textimpl_582 {
() => {
// Module: crate::element::text
// Provides: {"impl_582"}
// Dependencies: {}
impl < 'a , Coord : 'a , DB : DrawingBackend , T : Borrow < str > > Drawable < DB > for MultiLineText < 'a , Coord , T > { fn draw < I : Iterator < Item = BackendCoord > > (& self , mut points : I , backend : & mut DB , _ : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > { if let Some (a) = points . next () { for (point , text) in self . layout_lines (a) . zip (self . lines . iter ()) { backend . draw_text (text . borrow () , & self . style , point) ? ; } } Ok (()) } }
};
}
