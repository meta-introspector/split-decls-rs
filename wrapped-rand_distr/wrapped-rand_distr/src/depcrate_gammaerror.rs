// Generated macro for Error (enum)
macro_rules! Depcrate_gammaError {
() => {
// Module: crate::gamma
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type returned from [`Gamma::new`]."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Error { # [doc = " `shape <= 0` or `nan`."] ShapeTooSmall , # [doc = " `scale <= 0` or `nan`."] ScaleTooSmall , # [doc = " `1 / scale == 0`."] ScaleTooLarge , }
};
}
