// Generated macro for PointElement (trait)
macro_rules! Depcrate_element_pointsPointElement {
() => {
// Module: crate::element::points
// Provides: {"PointElement"}
// Dependencies: {}
# [doc = "\nA common trait for elements that can be interpreted as points: A cross, a circle, a triangle marker...\n\nThis is used internally by Plotters and should probably not be included in user code.\nSee [`EmptyElement`] for more information and examples.\n"] pub trait PointElement < Coord , Size : SizeDesc > { # [doc = "\n    Point creator.\n\n    This is used internally by Plotters and should probably not be included in user code.\n    See [`EmptyElement`] for more information and examples.\n    "] fn make_point (pos : Coord , size : Size , style : ShapeStyle) -> Self ; }
};
}
