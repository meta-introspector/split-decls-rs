// Generated macro for Error (enum)
macro_rules! Depcrate_weibullError {
() => {
// Module: crate::weibull
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type returned from [`Weibull::new`]."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Error { # [doc = " `scale <= 0` or `nan`."] ScaleTooSmall , # [doc = " `shape <= 0` or `nan`."] ShapeTooSmall , }
};
}
