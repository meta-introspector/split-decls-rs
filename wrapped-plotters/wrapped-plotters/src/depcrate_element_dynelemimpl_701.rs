// Generated macro for impl_701 (impl)
macro_rules! Depcrate_element_dynelemimpl_701 {
() => {
// Module: crate::element::dynelem
// Provides: {"impl_701"}
// Dependencies: {}
impl < 'b , T , DB , Coord > IntoDynElement < 'b , DB , Coord > for T where T : Drawable < DB > + 'b , for < 'a > & 'a T : PointCollection < 'a , Coord > , Coord : Clone , DB : DrawingBackend , { fn into_dyn (self) -> DynElement < 'b , DB , Coord > { DynElement { points : self . point_iter () . into_iter () . map (| x | x . borrow () . clone ()) . collect () , drawable : Box :: new (self) , } } }
};
}
