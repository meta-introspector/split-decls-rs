// Generated macro for impl_561 (impl)
macro_rules! Depcrate_element_basic_shapes_3dimpl_561 {
() => {
// Module: crate::element::basic_shapes_3d
// Provides: {"impl_561"}
// Dependencies: {}
impl < X : Clone , Y : Clone , Z : Clone > Cubiod < X , Y , Z > { # [doc = "\n    Creates a cuboid.\n\n    See [`Cubiod`] for more information and examples.\n    "] # [allow (clippy :: redundant_clone)] pub fn new < FS : Into < ShapeStyle > , ES : Into < ShapeStyle > > ([(x0 , y0 , z0) , (x1 , y1 , z1)] : [(X , Y , Z) ; 2] , face_style : FS , edge_style : ES ,) -> Self { Self { face_style : face_style . into () , edge_style : edge_style . into () , vert : [(x0 . clone () , y0 . clone () , z0 . clone ()) , (x0 . clone () , y0 . clone () , z1 . clone ()) , (x0 . clone () , y1 . clone () , z0 . clone ()) , (x0 . clone () , y1 . clone () , z1 . clone ()) , (x1 . clone () , y0 . clone () , z0 . clone ()) , (x1 . clone () , y0 . clone () , z1 . clone ()) , (x1 . clone () , y1 . clone () , z0 . clone ()) , (x1 . clone () , y1 . clone () , z1 . clone ()) ,] , } } }
};
}
