// Generated macro for impl_696 (impl)
macro_rules! Depcrate_element_dynelemimpl_696 {
() => {
// Module: crate::element::dynelem
// Provides: {"impl_696"}
// Dependencies: {}
impl < DB : DrawingBackend , T : Drawable < DB > > DynDrawable < DB > for T { fn draw_dyn (& self , points : & mut dyn Iterator < Item = BackendCoord > , backend : & mut DB , parent_dim : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > { T :: draw (self , points , backend , parent_dim) } }
};
}
