// Generated macro for impl_461 (impl)
macro_rules! Depcrateimpl_461 {
() => {
// Module: crate
// Provides: {"impl_461"}
// Dependencies: {}
impl fmt :: Display for Variance { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let description = match self { Variance :: Bivariant => "bivariant" , Variance :: Covariant => "covariant" , Variance :: Contravariant => "contravariant" , Variance :: Invariant => "invariant" , } ; f . pad (description) } }
};
}
