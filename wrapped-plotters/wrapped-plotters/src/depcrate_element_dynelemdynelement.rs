// Generated macro for DynElement (struct)
macro_rules! Depcrate_element_dynelemDynElement {
() => {
// Module: crate::element::dynelem
// Provides: {"DynElement"}
// Dependencies: {}
# [doc = " The container for a dynamically dispatched element"] pub struct DynElement < 'a , DB , Coord > where DB : DrawingBackend , Coord : Clone , { points : Vec < Coord > , drawable : Box < dyn DynDrawable < DB > + 'a > , }
};
}
