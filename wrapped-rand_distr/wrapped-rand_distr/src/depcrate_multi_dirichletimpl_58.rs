// Generated macro for impl_58 (impl)
macro_rules! Depcrate_multi_dirichletimpl_58 {
() => {
// Module: crate::multi::dirichlet
// Provides: {"impl_58"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Error :: AlphaTooShort | Error :: SizeTooSmall => { "less than 2 dimensions in Dirichlet distribution" } Error :: AlphaTooSmall => "alpha is not positive in Dirichlet distribution" , Error :: AlphaSubnormal => "alpha contains a subnormal value in Dirichlet distribution" , Error :: AlphaInfinite => "alpha contains an infinite value in Dirichlet distribution" , Error :: FailedToCreateGamma => { "failed to create required Gamma distribution for Dirichlet distribution" } Error :: FailedToCreateBeta => { "failed to create required Beta distribution for Dirichlet distribution" } }) } }
};
}
