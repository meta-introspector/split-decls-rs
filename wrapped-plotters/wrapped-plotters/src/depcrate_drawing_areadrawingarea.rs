// Generated macro for DrawingArea (struct)
macro_rules! Depcrate_drawing_areaDrawingArea {
() => {
// Module: crate::drawing::area
// Provides: {"DrawingArea"}
// Dependencies: {}
# [doc = " The abstraction of a drawing area. Plotters uses drawing area as the fundamental abstraction for the"] # [doc = " high level drawing API. The major functionality provided by the drawing area is"] # [doc = " 1. Layout specification - Split the parent drawing area into sub-drawing-areas"] # [doc = " 2. Coordinate Translation - Allows guest coordinate system attached and used for drawing."] # [doc = " 3. Element based drawing - drawing area provides the environment the element can be drawn onto it."] pub struct DrawingArea < DB : DrawingBackend , CT : CoordTranslate > { backend : Rc < RefCell < DB > > , rect : Rect , coord : CT , }
};
}
