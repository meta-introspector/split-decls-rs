// Generated macro for impl_2249 (impl)
macro_rules! Depcrate_geometry_isometryimpl_2249 {
() => {
// Module: crate::geometry::isometry
// Provides: {"impl_2249"}
// Dependencies: {}
impl < T : RealField + fmt :: Display , R , const D : usize > fmt :: Display for Isometry < T , R , D > where R : fmt :: Display , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let precision = f . precision () . unwrap_or (3) ; writeln ! (f , "Isometry {{") ? ; write ! (f , "{:.*}" , precision , self . translation) ? ; write ! (f , "{:.*}" , precision , self . rotation) ? ; writeln ! (f , "}}") } }
};
}
