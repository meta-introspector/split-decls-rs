// Generated macro for Error (enum)
macro_rules! Depcrate_paretoError {
() => {
// Module: crate::pareto
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type returned from [`Pareto::new`]."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Error { # [doc = " `scale <= 0` or `nan`."] ScaleTooSmall , # [doc = " `shape <= 0` or `nan`."] ShapeTooSmall , }
};
}
