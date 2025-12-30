// Generated macro for impl_531 (impl)
macro_rules! Depcrate_element_basic_shapesimpl_531 {
() => {
// Module: crate::element::basic_shapes
// Provides: {"impl_531"}
// Dependencies: {}
impl < I : Iterator + Clone , Size : SizeDesc > DashedPathElement < I , Size > { # [doc = " Create a new path"] # [doc = " - `points`: The iterator of the points"] # [doc = " - `size`: The dash size"] # [doc = " - `spacing`: The dash-to-dash spacing (gap size)"] # [doc = " - `style`: The shape style"] # [doc = " - returns the created element"] pub fn new < I0 , S > (points : I0 , size : Size , spacing : Size , style : S) -> Self where I0 : IntoIterator < IntoIter = I > , S : Into < ShapeStyle > , { Self { points : points . into_iter () , size , spacing , style : style . into () , } } }
};
}
