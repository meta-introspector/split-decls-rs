// Generated macro for impl_357 (impl)
macro_rules! Depcrate_poissonimpl_357 {
() => {
// Module: crate::poisson
// Provides: {"impl_357"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Error :: ShapeTooSmall => "lambda is not positive in Poisson distribution" , Error :: NonFinite => "lambda is infinite or nan in Poisson distribution" , Error :: ShapeTooLarge => { "lambda is too large in Poisson distribution, see Poisson::MAX_LAMBDA" } }) } }
};
}
