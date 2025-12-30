// Generated macro for ParameterErrorKind (enum)
macro_rules! Depcrate_errorParameterErrorKind {
() => {
// Module: crate::error
// Provides: {"ParameterErrorKind"}
// Dependencies: {}
# [doc = " Details how a parameter is malformed."] # [derive (Clone , Debug , Hash , PartialEq)] # [non_exhaustive] pub enum ParameterErrorKind { # [doc = " The dimensions passed are wrong."] DimensionMismatch , # [doc = " Repeated an operation for which error that could not be cloned was emitted already."] FailedAlready , # [doc = " A string describing the parameter."] # [doc = " This is discouraged and is likely to get deprecated (but not removed)."] Generic (String) , # [doc = " The end of the image has been reached."] NoMoreData , }
};
}
