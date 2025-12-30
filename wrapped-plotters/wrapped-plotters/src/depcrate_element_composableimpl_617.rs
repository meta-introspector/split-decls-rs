// Generated macro for impl_617 (impl)
macro_rules! Depcrate_element_composableimpl_617 {
() => {
// Module: crate::element::composable
// Provides: {"impl_617"}
// Dependencies: {}
impl < Coord , DB : DrawingBackend , A > Drawable < DB > for BoxedElement < Coord , DB , A > where for < 'a > & 'a A : PointCollection < 'a , BackendCoord > , A : Drawable < DB > , { fn draw < I : Iterator < Item = BackendCoord > > (& self , mut pos : I , backend : & mut DB , ps : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > { if let Some ((x0 , y0)) = pos . next () { self . inner . draw (self . inner . point_iter () . into_iter () . map (| p | { let p = p . borrow () ; (p . 0 + x0 , p . 1 + y0) }) , backend , ps ,) ? ; } Ok (()) } }
};
}
