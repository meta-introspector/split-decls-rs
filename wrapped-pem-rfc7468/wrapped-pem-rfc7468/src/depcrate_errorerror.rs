// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " PEM errors."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub enum Error { # [doc = " Base64-related errors."] Base64 (base64ct :: Error) , # [doc = " Character encoding-related errors."] CharacterEncoding , # [doc = " Errors in the encapsulated text (which aren't specifically Base64-related)."] EncapsulatedText , # [doc = " Header detected in the encapsulated text."] HeaderDisallowed , # [doc = " Invalid label."] Label , # [doc = " Invalid length."] Length , # [doc = " \"Preamble\" (text before pre-encapsulation boundary) contains invalid data."] Preamble , # [doc = " Errors in the pre-encapsulation boundary."] PreEncapsulationBoundary , # [doc = " Errors in the post-encapsulation boundary."] PostEncapsulationBoundary , # [doc = " Unexpected PEM type label."] UnexpectedTypeLabel { # [doc = " Type label that was expected."] expected : & 'static str , } , }
};
}
