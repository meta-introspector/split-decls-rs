// Generated macro for Error (enum)
macro_rules! Depcrate_repository_attributesError {
() => {
// Module: crate::repository::attributes
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`Repository::attributes()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] ConfigureAttributes (# [from] config :: attribute_stack :: Error) , # [error (transparent)] ConfigureExcludes (# [from] config :: exclude_stack :: Error) , }
};
}
