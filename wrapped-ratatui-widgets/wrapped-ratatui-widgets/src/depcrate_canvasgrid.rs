// Generated macro for Grid (trait)
macro_rules! Depcrate_canvasGrid {
() => {
// Module: crate::canvas
// Provides: {"Grid"}
// Dependencies: {}
# [doc = " A grid of cells that can be painted on."] # [doc = ""] # [doc = " The grid represents a particular screen region measured in rows and columns. The underlying"] # [doc = " resolution of the grid might exceed the number of rows and columns. For example, a grid of"] # [doc = " Braille patterns will have a resolution of 2x4 dots per cell. This means that a grid of 10x10"] # [doc = " cells will have a resolution of 20x40 dots."] trait Grid : fmt :: Debug { # [doc = " Get the resolution of the grid in number of dots."] # [doc = ""] # [doc = " This doesn't have to be the same as the number of rows and columns of the grid. For example,"] # [doc = " a grid of Braille patterns will have a resolution of 2x4 dots per cell. This means that a"] # [doc = " grid of 10x10 cells will have a resolution of 20x40 dots."] fn resolution (& self) -> (f64 , f64) ; # [doc = " Paint a point of the grid."] # [doc = ""] # [doc = " The point is expressed in number of dots starting at the origin of the grid in the top left"] # [doc = " corner. Note that this is not the same as the `(x, y)` coordinates of the canvas."] fn paint (& mut self , x : usize , y : usize , color : Color) ; # [doc = " Save the current state of the [`Grid`] as a layer to be rendered"] fn save (& self) -> Layer ; # [doc = " Reset the grid to its initial state"] fn reset (& mut self) ; }
};
}
