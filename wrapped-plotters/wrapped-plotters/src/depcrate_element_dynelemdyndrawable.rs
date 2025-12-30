// Generated macro for DynDrawable (trait)
macro_rules! Depcrate_element_dynelemDynDrawable {
() => {
// Module: crate::element::dynelem
// Provides: {"DynDrawable"}
// Dependencies: {}
trait DynDrawable < DB : DrawingBackend > { fn draw_dyn (& self , points : & mut dyn Iterator < Item = BackendCoord > , backend : & mut DB , parent_dim : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > ; }
};
}
