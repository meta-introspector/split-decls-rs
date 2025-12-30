// Generated macro for impl_141 (impl)
macro_rules! Depcrate_binomialimpl_141 {
() => {
// Module: crate::binomial
// Provides: {"impl_141"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Error :: ProbabilityTooSmall => "p < 0 or is NaN in binomial distribution" , Error :: ProbabilityTooLarge => "p > 1 in binomial distribution" , }) } }
};
}
