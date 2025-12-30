// Generated macro for ComposedElement (struct)
macro_rules! Depcrate_element_composableComposedElement {
() => {
// Module: crate::element::composable
// Provides: {"ComposedElement"}
// Dependencies: {}
# [doc = "\nA container for two drawable elements, used for composition.\n\nThis is used internally by Plotters and should probably not be included in user code.\nSee [`EmptyElement`] for more information and examples.\n"] pub struct ComposedElement < Coord , DB : DrawingBackend , A , B > where A : Drawable < DB > , B : Drawable < DB > , { first : A , second : B , offset : Coord , phantom : PhantomData < DB > , }
};
}
