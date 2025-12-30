// Generated macro for ValidationMode (enum)
macro_rules! Depcrate_validationValidationMode {
() => {
// Module: crate::validation
// Provides: {"ValidationMode"}
// Dependencies: {}
# [doc = " Validation mode"] # [derive (Copy , Clone , Debug)] pub enum ValidationMode { # [doc = " Execute all validation rules."] Strict , # [doc = " The executor itself also has error handling, so it can improve"] # [doc = " performance, but it can lose some error messages."] Fast , }
};
}
