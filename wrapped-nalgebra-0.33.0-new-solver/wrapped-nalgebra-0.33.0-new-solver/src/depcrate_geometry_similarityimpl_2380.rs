// Generated macro for impl_2380 (impl)
macro_rules! Depcrate_geometry_similarityimpl_2380 {
() => {
// Module: crate::geometry::similarity
// Provides: {"impl_2380"}
// Dependencies: {}
impl < T , R , const D : usize > fmt :: Display for Similarity < T , R , D > where T : RealField + fmt :: Display , R : AbstractRotation < T , D > + fmt :: Display , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let precision = f . precision () . unwrap_or (3) ; writeln ! (f , "Similarity {{") ? ; write ! (f , "{:.*}" , precision , self . isometry) ? ; write ! (f , "Scaling: {:.*}" , precision , self . scaling) ? ; writeln ! (f , "}}") } }
};
}
