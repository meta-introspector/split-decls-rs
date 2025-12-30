// Generated macro for Context (struct)
macro_rules! Depcrate_canvasContext {
() => {
// Module: crate::canvas
// Provides: {"Context"}
// Dependencies: {}
# [doc = " Holds the state of the [`Canvas`] when painting to it."] # [doc = ""] # [doc = " This is used by the [`Canvas`] widget to draw shapes on the grid. It can be useful to think of"] # [doc = " this as similar to the `Frame` struct that is used to draw widgets on the terminal."] # [derive (Debug)] pub struct Context < 'a > { x_bounds : [f64 ; 2] , y_bounds : [f64 ; 2] , grid : Box < dyn Grid > , dirty : bool , layers : Vec < Layer > , labels : Vec < Label < 'a > > , }
};
}
