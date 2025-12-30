// Generated macro for DottedPathElement (struct)
macro_rules! Depcrate_element_basic_shapesDottedPathElement {
() => {
// Module: crate::element::basic_shapes
// Provides: {"DottedPathElement"}
// Dependencies: {}
# [doc = " An element of a series of connected lines in dot style for any markers."] # [doc = ""] # [doc = " It's similar to [`PathElement`] but use a marker function to draw markers with spacing."] pub struct DottedPathElement < I : Iterator + Clone , Size : SizeDesc , Marker > { points : I , shift : Size , spacing : Size , func : Box < dyn Fn (BackendCoord) -> Marker > , }
};
}
