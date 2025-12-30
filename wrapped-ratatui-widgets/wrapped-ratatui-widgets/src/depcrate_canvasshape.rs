// Generated macro for Shape (trait)
macro_rules! Depcrate_canvasShape {
() => {
// Module: crate::canvas
// Provides: {"Shape"}
// Dependencies: {}
# [doc = " Something that can be drawn on a [`Canvas`]."] # [doc = ""] # [doc = " You may implement your own canvas custom widgets by implementing this trait."] pub trait Shape { # [doc = " Draws this [`Shape`] using the given [`Painter`]."] # [doc = ""] # [doc = " This is the only method required to implement a custom widget that can be drawn on a"] # [doc = " [`Canvas`]."] fn draw (& self , painter : & mut Painter) ; }
};
}
