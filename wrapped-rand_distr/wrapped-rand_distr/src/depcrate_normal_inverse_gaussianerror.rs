// Generated macro for Error (enum)
macro_rules! Depcrate_normal_inverse_gaussianError {
() => {
// Module: crate::normal_inverse_gaussian
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type returned from [`NormalInverseGaussian::new`]"] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum Error { # [doc = " `alpha <= 0` or `nan`."] AlphaNegativeOrNull , # [doc = " `|beta| >= alpha` or `nan`."] AbsoluteBetaNotLessThanAlpha , }
};
}
