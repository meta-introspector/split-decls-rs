// Generated macro for impl_2021 (impl)
macro_rules! Depcrate_geometry_translationimpl_2021 {
() => {
// Module: crate::geometry::translation
// Provides: {"impl_2021"}
// Dependencies: {}
impl < T : Scalar + fmt :: Display , const D : usize > fmt :: Display for Translation < T , D > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let precision = f . precision () . unwrap_or (3) ; writeln ! (f , "Translation {{") ? ; write ! (f , "{:.*}" , precision , self . vector) ? ; writeln ! (f , "}}") } }
};
}
