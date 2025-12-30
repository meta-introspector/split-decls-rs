// Generated macro for impl_273 (impl)
macro_rules! Depcrate_hypergeometricimpl_273 {
() => {
// Module: crate::hypergeometric
// Provides: {"impl_273"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Error :: PopulationTooLarge => { "total_population_size is too large causing underflow in geometric distribution" } Error :: ProbabilityTooLarge => { "population_with_feature > total_population_size in geometric distribution" } Error :: SampleSizeTooLarge => { "sample_size > total_population_size in geometric distribution" } }) } }
};
}
