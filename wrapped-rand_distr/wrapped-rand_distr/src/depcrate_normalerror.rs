// Generated macro for Error (enum)
macro_rules! Depcrate_normalError {
() => {
// Module: crate::normal
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type returned from [`Normal::new`] and [`LogNormal::new`](crate::LogNormal::new)."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Error { # [doc = " The mean value is too small (log-normal samples must be positive)"] MeanTooSmall , # [doc = " The standard deviation or other dispersion parameter is not finite."] BadVariance , }
};
}
