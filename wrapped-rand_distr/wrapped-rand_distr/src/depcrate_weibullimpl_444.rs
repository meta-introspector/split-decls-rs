// Generated macro for impl_444 (impl)
macro_rules! Depcrate_weibullimpl_444 {
() => {
// Module: crate::weibull
// Provides: {"impl_444"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Error :: ScaleTooSmall => "scale is not positive in Weibull distribution" , Error :: ShapeTooSmall => "shape is not positive in Weibull distribution" , }) } }
};
}
