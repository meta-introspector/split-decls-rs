// Generated macro for IntoDrawingArea (trait)
macro_rules! Depcrate_drawing_areaIntoDrawingArea {
() => {
// Module: crate::drawing::area
// Provides: {"IntoDrawingArea"}
// Dependencies: {}
# [doc = " A type which can be converted into a root drawing area"] pub trait IntoDrawingArea : DrawingBackend + Sized { # [doc = " Convert the type into a root drawing area"] fn into_drawing_area (self) -> DrawingArea < Self , Shift > ; }
};
}
