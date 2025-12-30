// Generated macro for impl_200 (impl)
macro_rules! Depcrate_fisher_fimpl_200 {
() => {
// Module: crate::fisher_f
// Provides: {"impl_200"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Error :: MTooSmall => "m is not positive in Fisher F distribution" , Error :: NTooSmall => "n is not positive in Fisher F distribution" , }) } }
};
}
