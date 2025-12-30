// Generated macro for impl_287 (impl)
macro_rules! Depcrate_inverse_gaussianimpl_287 {
() => {
// Module: crate::inverse_gaussian
// Provides: {"impl_287"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Error :: MeanNegativeOrNull => "mean <= 0 or is NaN in inverse Gaussian distribution" , Error :: ShapeNegativeOrNull => "shape <= 0 or is NaN in inverse Gaussian distribution" , }) } }
};
}
