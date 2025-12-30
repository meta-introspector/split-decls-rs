// Generated macro for impl_622 (impl)
macro_rules! Depcrate_element_composableimpl_622 {
() => {
// Module: crate::element::composable
// Provides: {"impl_622"}
// Dependencies: {}
impl < Coord , DB : DrawingBackend , A , B , C > Add < C > for ComposedElement < Coord , DB , A , B > where A : Drawable < DB > , for < 'a > & 'a A : PointCollection < 'a , BackendCoord > , B : Drawable < DB > , for < 'a > & 'a B : PointCollection < 'a , BackendCoord > , C : Drawable < DB > , for < 'a > & 'a C : PointCollection < 'a , BackendCoord > , { type Output = ComposedElement < Coord , DB , A , ComposedElement < BackendCoord , DB , B , C > > ; fn add (self , rhs : C) -> Self :: Output { ComposedElement { offset : self . offset , first : self . first , second : ComposedElement { offset : (0 , 0) , first : self . second , second : rhs , phantom : PhantomData , } , phantom : PhantomData , } } }
};
}
