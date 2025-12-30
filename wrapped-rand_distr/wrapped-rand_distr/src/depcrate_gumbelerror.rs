// Generated macro for Error (enum)
macro_rules! Depcrate_gumbelError {
() => {
// Module: crate::gumbel
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type returned from [`Gumbel::new`]."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Error { # [doc = " location is infinite or NaN"] LocationNotFinite , # [doc = " scale is not finite positive number"] ScaleNotPositive , }
};
}
