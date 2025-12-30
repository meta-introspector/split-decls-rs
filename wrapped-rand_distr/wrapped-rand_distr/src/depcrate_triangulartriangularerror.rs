// Generated macro for TriangularError (enum)
macro_rules! Depcrate_triangularTriangularError {
() => {
// Module: crate::triangular
// Provides: {"TriangularError"}
// Dependencies: {}
# [doc = " Error type returned from [`Triangular::new`]."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum TriangularError { # [doc = " `max < min` or `min` or `max` is NaN."] RangeTooSmall , # [doc = " `mode < min` or `mode > max` or `mode` is NaN."] ModeRange , }
};
}
