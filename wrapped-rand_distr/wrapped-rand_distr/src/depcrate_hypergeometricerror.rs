// Generated macro for Error (enum)
macro_rules! Depcrate_hypergeometricError {
() => {
// Module: crate::hypergeometric
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type returned from [`Hypergeometric::new`]."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Error { # [doc = " `total_population_size` is too large, causing floating point underflow."] PopulationTooLarge , # [doc = " `population_with_feature > total_population_size`."] ProbabilityTooLarge , # [doc = " `sample_size > total_population_size`."] SampleSizeTooLarge , }
};
}
