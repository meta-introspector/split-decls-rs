// Generated macro for impl_125 (impl)
macro_rules! Depcrate_betaimpl_125 {
() => {
// Module: crate::beta
// Provides: {"impl_125"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Error :: AlphaTooSmall => "alpha is not positive in beta distribution" , Error :: BetaTooSmall => "beta is not positive in beta distribution" , }) } }
};
}
