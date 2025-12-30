// Generated macro for impl_621 (impl)
macro_rules! Depcrate_element_composableimpl_621 {
() => {
// Module: crate::element::composable
// Provides: {"impl_621"}
// Dependencies: {}
impl < Coord , DB : DrawingBackend , A , B > Drawable < DB > for ComposedElement < Coord , DB , A , B > where for < 'a > & 'a A : PointCollection < 'a , BackendCoord > , for < 'b > & 'b B : PointCollection < 'b , BackendCoord > , A : Drawable < DB > , B : Drawable < DB > , { fn draw < I : Iterator < Item = BackendCoord > > (& self , mut pos : I , backend : & mut DB , ps : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > { if let Some ((x0 , y0)) = pos . next () { self . first . draw (self . first . point_iter () . into_iter () . map (| p | { let p = p . borrow () ; (p . 0 + x0 , p . 1 + y0) }) , backend , ps ,) ? ; self . second . draw (self . second . point_iter () . into_iter () . map (| p | { let p = p . borrow () ; (p . 0 + x0 , p . 1 + y0) }) , backend , ps ,) ? ; } Ok (()) } }
};
}
