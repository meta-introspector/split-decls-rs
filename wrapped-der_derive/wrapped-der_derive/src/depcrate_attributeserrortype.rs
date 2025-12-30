// Generated macro for ErrorType (enum)
macro_rules! Depcrate_attributesErrorType {
() => {
// Module: crate::attributes
// Provides: {"ErrorType"}
// Dependencies: {}
# [doc = " Error type used by the structure"] # [derive (Debug , Clone , Default , Eq , PartialEq)] pub (crate) enum ErrorType { # [doc = " Represents the ::der::Error type"] # [default] Der , # [doc = " Represents an error designed by Path"] Custom (Path) , }
};
}
