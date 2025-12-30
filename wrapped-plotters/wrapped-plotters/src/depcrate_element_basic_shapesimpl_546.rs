// Generated macro for impl_546 (impl)
macro_rules! Depcrate_element_basic_shapesimpl_546 {
() => {
// Module: crate::element::basic_shapes
// Provides: {"impl_546"}
// Dependencies: {}
impl < Coord , Size : SizeDesc > Circle < Coord , Size > { # [doc = " Create a new circle element"] # [doc = " - `coord` The center of the circle"] # [doc = " - `size` The radius of the circle"] # [doc = " - `style` The style of the circle"] # [doc = " - Return: The newly created circle element"] pub fn new < S : Into < ShapeStyle > > (coord : Coord , size : Size , style : S) -> Self { Self { center : coord , size , style : style . into () , } } }
};
}
