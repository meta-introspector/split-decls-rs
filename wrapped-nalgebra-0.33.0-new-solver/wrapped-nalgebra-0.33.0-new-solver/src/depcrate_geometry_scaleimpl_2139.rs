// Generated macro for impl_2139 (impl)
macro_rules! Depcrate_geometry_scaleimpl_2139 {
() => {
// Module: crate::geometry::scale
// Provides: {"impl_2139"}
// Dependencies: {}
impl < T : Scalar + fmt :: Display , const D : usize > fmt :: Display for Scale < T , D > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let precision = f . precision () . unwrap_or (3) ; writeln ! (f , "Scale {{") ? ; write ! (f , "{:.*}" , precision , self . vector) ? ; writeln ! (f , "}}") } }
};
}
