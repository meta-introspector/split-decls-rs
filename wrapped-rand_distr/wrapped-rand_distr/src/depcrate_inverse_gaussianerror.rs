// Generated macro for Error (enum)
macro_rules! Depcrate_inverse_gaussianError {
() => {
// Module: crate::inverse_gaussian
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type returned from [`InverseGaussian::new`]"] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum Error { # [doc = " `mean <= 0` or `nan`."] MeanNegativeOrNull , # [doc = " `shape <= 0` or `nan`."] ShapeNegativeOrNull , }
};
}
