// Generated macro for ValidationError (enum)
macro_rules! Depcrate_validatorValidationError {
() => {
// Module: crate::validator
// Provides: {"ValidationError"}
// Dependencies: {}
# [derive (Error , Debug , Clone)] pub enum ValidationError { # [error ("Invalid entity identifier: {message}")] InvalidEntityIdentifier { message : String } , # [error ("Data access error: {message}")] DataAccessError { message : String } , # [error ("Metadata not found for {entity}")] MetadataNotFound { entity : String } , # [error ("Cache error: {message}")] CacheError { message : String } , # [error ("Invalid input: {0}")] InvalidInput (String) , # [error ("Processing error: {0}")] ProcessingError (String) , }
};
}
