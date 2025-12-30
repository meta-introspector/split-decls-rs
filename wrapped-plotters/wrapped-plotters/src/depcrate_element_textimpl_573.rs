// Generated macro for impl_573 (impl)
macro_rules! Depcrate_element_textimpl_573 {
() => {
// Module: crate::element::text
// Provides: {"impl_573"}
// Dependencies: {}
impl < 'a , Coord : 'a , DB : DrawingBackend , T : Borrow < str > > Drawable < DB > for Text < 'a , Coord , T > { fn draw < I : Iterator < Item = BackendCoord > > (& self , mut points : I , backend : & mut DB , _ : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > { if let Some (a) = points . next () { return backend . draw_text (self . text . borrow () , & self . style , a) ; } Ok (()) } }
};
}
