// Generated macro for impl_612 (impl)
macro_rules! Depcrate_element_composableimpl_612 {
() => {
// Module: crate::element::composable
// Provides: {"impl_612"}
// Dependencies: {}
impl < Coord , Other , DB : DrawingBackend > Add < Other > for EmptyElement < Coord , DB > where Other : Drawable < DB > , for < 'a > & 'a Other : PointCollection < 'a , BackendCoord > , { type Output = BoxedElement < Coord , DB , Other > ; fn add (self , other : Other) -> Self :: Output { BoxedElement { offset : self . coord , inner : other , phantom : PhantomData , } } }
};
}
