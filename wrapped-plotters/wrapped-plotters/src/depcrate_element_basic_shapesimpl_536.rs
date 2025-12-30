// Generated macro for impl_536 (impl)
macro_rules! Depcrate_element_basic_shapesimpl_536 {
() => {
// Module: crate::element::basic_shapes
// Provides: {"impl_536"}
// Dependencies: {}
impl < I : Iterator + Clone , Size : SizeDesc , Marker > DottedPathElement < I , Size , Marker > { # [doc = " Create a new path"] # [doc = " - `points`: The iterator of the points"] # [doc = " - `shift`: The shift of the first marker"] # [doc = " - `spacing`: The spacing between markers"] # [doc = " - `func`: The marker function"] # [doc = " - returns the created element"] pub fn new < I0 , F > (points : I0 , shift : Size , spacing : Size , func : F) -> Self where I0 : IntoIterator < IntoIter = I > , F : Fn (BackendCoord) -> Marker + 'static , { Self { points : points . into_iter () , shift , spacing , func : Box :: new (func) , } } }
};
}
