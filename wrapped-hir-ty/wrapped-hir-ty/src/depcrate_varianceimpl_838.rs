// Generated macro for impl_838 (impl)
macro_rules! Depcrate_varianceimpl_838 {
() => {
// Module: crate::variance
// Provides: {"impl_838"}
// Dependencies: {}
impl fmt :: Display for Variance { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Variance :: Covariant => write ! (f , "covariant") , Variance :: Invariant => write ! (f , "invariant") , Variance :: Contravariant => write ! (f , "contravariant") , Variance :: Bivariant => write ! (f , "bivariant") , } } }
};
}
