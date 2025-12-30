// Generated macro for PatternError (enum)
macro_rules! Depcrate_errorPatternError {
() => {
// Module: crate::error
// Provides: {"PatternError"}
// Dependencies: {}
# [derive (Debug , Display , Copy , Clone , PartialEq , Eq)] # [non_exhaustive] pub enum PatternError { # [displaydoc ("Syntax error in pattern string or invalid serialized pattern")] InvalidPattern , # [displaydoc ("A placeholder is invalid or out of range for the pattern type")] InvalidPlaceholder , }
};
}
