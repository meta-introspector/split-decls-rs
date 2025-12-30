// Generated macro for VerifierError (struct)
macro_rules! Depcrate_verifierVerifierError {
() => {
// Module: crate::verifier
// Provides: {"VerifierError"}
// Dependencies: {}
# [doc = " A verifier error."] # [derive (Debug , PartialEq , Eq , Clone)] pub struct VerifierError { # [doc = " The entity causing the verifier error."] pub location : AnyEntity , # [doc = " Optionally provide some context for the given location; e.g., for `inst42` provide"] # [doc = " `Some(\"v3 = iconst.i32 0\")` for more comprehensible errors."] pub context : Option < String > , # [doc = " The error message."] pub message : String , }
};
}
