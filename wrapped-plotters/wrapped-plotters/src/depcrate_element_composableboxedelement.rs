// Generated macro for BoxedElement (struct)
macro_rules! Depcrate_element_composableBoxedElement {
() => {
// Module: crate::element::composable
// Provides: {"BoxedElement"}
// Dependencies: {}
# [doc = "\nA container for one drawable element, used for composition.\n\nThis is used internally by Plotters and should probably not be included in user code.\nSee [`EmptyElement`] for more information and examples.\n"] pub struct BoxedElement < Coord , DB : DrawingBackend , A : Drawable < DB > > { inner : A , offset : Coord , phantom : PhantomData < DB > , }
};
}
