// Generated macro for impl_318 (impl)
macro_rules! Depcrate_normal_inverse_gaussianimpl_318 {
() => {
// Module: crate::normal_inverse_gaussian
// Provides: {"impl_318"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Error :: AlphaNegativeOrNull => { "alpha <= 0 or is NaN in normal inverse Gaussian distribution" } Error :: AbsoluteBetaNotLessThanAlpha => { "|beta| >= alpha or is NaN in normal inverse Gaussian distribution" } }) } }
};
}
