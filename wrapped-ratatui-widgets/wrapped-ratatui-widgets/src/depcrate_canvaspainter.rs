// Generated macro for Painter (struct)
macro_rules! Depcrate_canvasPainter {
() => {
// Module: crate::canvas
// Provides: {"Painter"}
// Dependencies: {}
# [doc = " Painter is an abstraction over the [`Context`] that allows to draw shapes on the grid."] # [doc = ""] # [doc = " It is used by the [`Shape`] trait to draw shapes on the grid. It can be useful to think of this"] # [doc = " as similar to the [`Buffer`] struct that is used to draw widgets on the terminal."] # [derive (Debug)] pub struct Painter < 'a , 'b > { context : & 'a mut Context < 'b > , resolution : (f64 , f64) , }
};
}
