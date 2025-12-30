// Generated macro for VerboseErrorKind (enum)
macro_rules! Depcrate_errorVerboseErrorKind {
() => {
// Module: crate::error
// Provides: {"VerboseErrorKind"}
// Dependencies: {}
# [derive (Clone , Debug , Eq , PartialEq)] # [doc = " Error context for `VerboseError`"] pub enum VerboseErrorKind { # [doc = " Static string added by the `context` function"] Context (& 'static str) , # [doc = " Indicates which character was expected by the `char` function"] Char (char) , # [doc = " Error kind given by various nom parsers"] Nom (ErrorKind) , }
};
}
