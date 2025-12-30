// Generated macro for impl_618 (impl)
macro_rules! Depcrate_element_composableimpl_618 {
() => {
// Module: crate::element::composable
// Provides: {"impl_618"}
// Dependencies: {}
impl < Coord , DB : DrawingBackend , My , Yours > Add < Yours > for BoxedElement < Coord , DB , My > where My : Drawable < DB > , for < 'a > & 'a My : PointCollection < 'a , BackendCoord > , Yours : Drawable < DB > , for < 'a > & 'a Yours : PointCollection < 'a , BackendCoord > , { type Output = ComposedElement < Coord , DB , My , Yours > ; fn add (self , yours : Yours) -> Self :: Output { ComposedElement { offset : self . offset , first : self . inner , second : yours , phantom : PhantomData , } } }
};
}
