// Generated macro for VerboseError (struct)
macro_rules! Depcrate_errorVerboseError {
() => {
// Module: crate::error
// Provides: {"VerboseError"}
// Dependencies: {}
# [doc = " This error type accumulates errors and their position when backtracking"] # [doc = " through a parse tree. With some post processing,"] # [doc = " it can be used to display user friendly error messages"] # [derive (Clone , Debug , Eq , PartialEq)] pub struct VerboseError < I > { # [doc = " List of errors accumulated by `VerboseError`, containing the affected"] # [doc = " part of input data, and some context"] pub errors : Vec < (I , VerboseErrorKind) > , }
};
}
