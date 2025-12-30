// Generated macro for Error (enum)
macro_rules! Depcrate_poissonError {
() => {
// Module: crate::poisson
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type returned from [`Poisson::new`]."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Error { # [doc = " `lambda <= 0`"] ShapeTooSmall , # [doc = " `lambda = ∞` or `lambda = nan`"] NonFinite , # [doc = " `lambda` is too large, see [Poisson::MAX_LAMBDA]"] ShapeTooLarge , }
};
}
