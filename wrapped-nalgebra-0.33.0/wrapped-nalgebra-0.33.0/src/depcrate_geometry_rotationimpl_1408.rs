// Generated macro for impl_1408 (impl)
macro_rules! Depcrate_geometry_rotationimpl_1408 {
() => {
// Module: crate::geometry::rotation
// Provides: {"impl_1408"}
// Dependencies: {}
impl < T , const D : usize > fmt :: Display for Rotation < T , D > where T : RealField + fmt :: Display , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let precision = f . precision () . unwrap_or (3) ; writeln ! (f , "Rotation matrix {{") ? ; write ! (f , "{:.*}" , precision , self . matrix) ? ; writeln ! (f , "}}") } }
};
}
