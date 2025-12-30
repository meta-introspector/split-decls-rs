// Generated macro for impl_259 (impl)
macro_rules! Depcrate_gumbelimpl_259 {
() => {
// Module: crate::gumbel
// Provides: {"impl_259"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Error :: ScaleNotPositive => "scale is not positive and finite in Gumbel distribution" , Error :: LocationNotFinite => "location is not finite in Gumbel distribution" , }) } }
};
}
