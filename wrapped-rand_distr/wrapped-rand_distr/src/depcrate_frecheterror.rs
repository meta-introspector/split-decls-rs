// Generated macro for Error (enum)
macro_rules! Depcrate_frechetError {
() => {
// Module: crate::frechet
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type returned from [`Frechet::new`]."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Error { # [doc = " location is infinite or NaN"] LocationNotFinite , # [doc = " scale is not finite positive number"] ScaleNotPositive , # [doc = " shape is not finite positive number"] ShapeNotPositive , }
};
}
