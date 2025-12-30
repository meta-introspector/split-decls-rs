// Generated macro for impl_395 (impl)
macro_rules! Depcrate_coord_ranged2d_cartesianimpl_395 {
() => {
// Module: crate::coord::ranged2d::cartesian
// Provides: {"impl_395"}
// Dependencies: {}
impl < 'a , X : Ranged , Y : Ranged > MeshLine < 'a , X , Y > { # [doc = " Draw a single mesh line onto the backend"] pub fn draw < DB : DrawingBackend > (& self , backend : & mut DB , style : & ShapeStyle ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > { let (& left , & right) = match self { MeshLine :: XMesh (a , b , _) => (a , b) , MeshLine :: YMesh (a , b , _) => (a , b) , } ; backend . draw_line (left , right , style) } }
};
}
