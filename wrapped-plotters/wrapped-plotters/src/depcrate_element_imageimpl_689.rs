// Generated macro for impl_689 (impl)
macro_rules! Depcrate_element_imageimpl_689 {
() => {
// Module: crate::element::image
// Provides: {"impl_689"}
// Dependencies: {}
impl < 'a , Coord , DB : DrawingBackend > Drawable < DB > for BitMapElement < 'a , Coord > { fn draw < I : Iterator < Item = BackendCoord > > (& self , mut points : I , backend : & mut DB , _ : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > { if let Some ((x , y)) = points . next () { return backend . blit_bitmap ((x , y) , self . size , self . image . as_ref ()) ; } Ok (()) } }
};
}
