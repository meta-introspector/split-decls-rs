// Generated macro for impl_541 (impl)
macro_rules! Depcrate_element_basic_shapesimpl_541 {
() => {
// Module: crate::element::basic_shapes
// Provides: {"impl_541"}
// Dependencies: {}
impl < Coord > Rectangle < Coord > { # [doc = " Create a new path"] # [doc = " - `points`: The left upper and right lower corner of the rectangle"] # [doc = " - `style`: The shape style"] # [doc = " - returns the created element"] pub fn new < S : Into < ShapeStyle > > (points : [Coord ; 2] , style : S) -> Self { Self { points , style : style . into () , margin : (0 , 0 , 0 , 0) , } } # [doc = " Set the margin of the rectangle"] # [doc = " - `t`: The top margin"] # [doc = " - `b`: The bottom margin"] # [doc = " - `l`: The left margin"] # [doc = " - `r`: The right margin"] pub fn set_margin (& mut self , t : u32 , b : u32 , l : u32 , r : u32) -> & mut Self { self . margin = (t , b , l , r) ; self } # [doc = " Get the points of the rectangle"] # [doc = " - returns the element points"] pub fn get_points (& self) -> (& Coord , & Coord) { (& self . points [0] , & self . points [1]) } # [doc = " Set the style of the rectangle"] # [doc = " - `style`: The shape style"] # [doc = " - returns a mut reference to the rectangle"] pub fn set_style < S : Into < ShapeStyle > > (& mut self , style : S) -> & mut Self { self . style = style . into () ; self } }
};
}
