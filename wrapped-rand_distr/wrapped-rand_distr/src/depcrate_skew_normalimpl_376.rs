// Generated macro for impl_376 (impl)
macro_rules! Depcrate_skew_normalimpl_376 {
() => {
// Module: crate::skew_normal
// Provides: {"impl_376"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Error :: ScaleTooSmall => { "scale parameter is either non-finite or it is less or equal to zero in skew normal distribution" } Error :: BadShape => "shape parameter is non-finite in skew normal distribution" , }) } }
};
}
