// Generated macro for impl_614 (impl)
macro_rules! Depcrate_element_composableimpl_614 {
() => {
// Module: crate::element::composable
// Provides: {"impl_614"}
// Dependencies: {}
impl < Coord , DB : DrawingBackend > Drawable < DB > for EmptyElement < Coord , DB > { fn draw < I : Iterator < Item = BackendCoord > > (& self , _pos : I , _backend : & mut DB , _ : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > { Ok (()) } }
};
}
