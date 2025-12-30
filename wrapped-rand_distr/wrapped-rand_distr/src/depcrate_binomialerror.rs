// Generated macro for Error (enum)
macro_rules! Depcrate_binomialError {
() => {
// Module: crate::binomial
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type returned from [`Binomial::new`]."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [non_exhaustive] pub enum Error { # [doc = " `p < 0` or `nan`."] ProbabilityTooSmall , # [doc = " `p > 1`."] ProbabilityTooLarge , }
};
}
