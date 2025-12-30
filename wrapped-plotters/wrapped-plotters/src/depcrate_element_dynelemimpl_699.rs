// Generated macro for impl_699 (impl)
macro_rules! Depcrate_element_dynelemimpl_699 {
() => {
// Module: crate::element::dynelem
// Provides: {"impl_699"}
// Dependencies: {}
impl < 'a , DB : DrawingBackend , Coord : Clone > Drawable < DB > for DynElement < 'a , DB , Coord > { fn draw < I : Iterator < Item = BackendCoord > > (& self , mut pos : I , backend : & mut DB , parent_dim : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > { self . drawable . draw_dyn (& mut pos , backend , parent_dim) } }
};
}
