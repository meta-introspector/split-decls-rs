// Generated macro for impl_304 (impl)
macro_rules! Depcrate_normalimpl_304 {
() => {
// Module: crate::normal
// Provides: {"impl_304"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Error :: MeanTooSmall => "mean < 0 or NaN in log-normal distribution" , Error :: BadVariance => "variation parameter is non-finite in (log)normal distribution" , }) } }
};
}
