// Generated macro for NameValidationError (struct)
macro_rules! Depcrate_restricted_namesNameValidationError {
() => {
// Module: crate::restricted_names
// Provides: {"NameValidationError"}
// Dependencies: {}
# [doc = " Error validating names in Cargo."] # [derive (Debug , thiserror :: Error)] # [error (transparent)] pub struct NameValidationError (# [from] ErrorKind) ;
};
}
