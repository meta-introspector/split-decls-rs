// Generated macro for Error (enum)
macro_rules! Depcrate_skew_normalError {
() => {
// Module: crate::skew_normal
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type returned from [`SkewNormal::new`]."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Error { # [doc = " The scale parameter is not finite or it is less or equal to zero."] ScaleTooSmall , # [doc = " The shape parameter is not finite."] BadShape , }
};
}
