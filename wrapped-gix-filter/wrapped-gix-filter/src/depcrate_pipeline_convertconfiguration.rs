// Generated macro for configuration (module)
macro_rules! Depcrate_pipeline_convertconfiguration {
() => {
// Module: crate::pipeline::convert
// Provides: {"configuration"}
// Dependencies: {}
# [doc = ""] pub mod configuration { use bstr :: BString ; # [doc = " Errors related to the configuration of filter attributes."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("The encoding named '{name}' isn't available")] UnknownEncoding { name : BString } , # [error ("Encodings must be names, like UTF-16, and cannot be booleans.")] InvalidEncoding , } }
};
}
