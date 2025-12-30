// Generated macro for PertError (enum)
macro_rules! Depcrate_pertPertError {
() => {
// Module: crate::pert
// Provides: {"PertError"}
// Dependencies: {}
# [doc = " Error type returned from [`Pert`] constructors."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum PertError { # [doc = " `max < min` or `min` or `max` is NaN."] RangeTooSmall , # [doc = " `mode < min` or `mode > max` or `mode` is NaN."] ModeRange , # [doc = " `shape < 0` or `shape` is NaN"] ShapeTooSmall , }
};
}
