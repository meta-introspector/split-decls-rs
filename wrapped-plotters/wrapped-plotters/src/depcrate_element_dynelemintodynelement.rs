// Generated macro for IntoDynElement (trait)
macro_rules! Depcrate_element_dynelemIntoDynElement {
() => {
// Module: crate::element::dynelem
// Provides: {"IntoDynElement"}
// Dependencies: {}
# [doc = " The trait that makes the conversion from the statically dispatched element"] # [doc = " to the dynamically dispatched element"] pub trait IntoDynElement < 'a , DB : DrawingBackend , Coord : Clone > where Self : 'a , { # [doc = " Make the conversion"] fn into_dyn (self) -> DynElement < 'a , DB , Coord > ; }
};
}
