// Generated macro for DashedPathElement (struct)
macro_rules! Depcrate_element_basic_shapesDashedPathElement {
() => {
// Module: crate::element::basic_shapes
// Provides: {"DashedPathElement"}
// Dependencies: {}
# [doc = " An element of a series of connected lines in dash style."] # [doc = ""] # [doc = " It's similar to [`PathElement`] but has a dash style."] pub struct DashedPathElement < I : Iterator + Clone , Size : SizeDesc > { points : I , size : Size , spacing : Size , style : ShapeStyle , }
};
}
