// Generated macro for key (module)
macro_rules! Depcrate_config_treekey {
() => {
// Module: crate::config::tree
// Provides: {"key"}
// Dependencies: {}
# [doc = ""] pub mod key { # [doc = ""] pub mod validate { # [doc = " The error returned by [`Key::validate()`][crate::config::tree::Key::validate()]."] # [derive (Debug , thiserror :: Error)] # [error (transparent)] # [allow (missing_docs)] pub struct Error { # [from] source : Box < dyn std :: error :: Error + Send + Sync + 'static > , } } # [doc = ""] pub mod validate_assignment { # [doc = " The error returned by [`Key::validated_assignment`*()][crate::config::tree::Key::validated_assignment_fmt()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Failed to validate the value to be assigned to this key")] Validate (# [from] super :: validate :: Error) , # [error ("{message}")] Name { message : String } , } } }
};
}
