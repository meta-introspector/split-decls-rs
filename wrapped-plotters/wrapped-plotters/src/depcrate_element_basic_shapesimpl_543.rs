// Generated macro for impl_543 (impl)
macro_rules! Depcrate_element_basic_shapesimpl_543 {
() => {
// Module: crate::element::basic_shapes
// Provides: {"impl_543"}
// Dependencies: {}
impl < Coord , DB : DrawingBackend > Drawable < DB > for Rectangle < Coord > { fn draw < I : Iterator < Item = BackendCoord > > (& self , mut points : I , backend : & mut DB , _ : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > { match (points . next () , points . next ()) { (Some (a) , Some (b)) => { let (mut a , mut b) = ((a . 0 . min (b . 0) , a . 1 . min (b . 1)) , (a . 0 . max (b . 0) , a . 1 . max (b . 1))) ; a . 1 += self . margin . 0 as i32 ; b . 1 -= self . margin . 1 as i32 ; a . 0 += self . margin . 2 as i32 ; b . 0 -= self . margin . 3 as i32 ; backend . draw_rect (a , b , & self . style , self . style . filled) } _ => Ok (()) , } } }
};
}
