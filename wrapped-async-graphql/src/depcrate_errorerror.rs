// Generated macro for Error (struct)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An error with a message and optional extensions."] # [derive (Clone , Serialize)] pub struct Error { # [doc = " The error message."] pub message : String , # [doc = " The source of the error."] # [serde (skip)] pub source : Option < Arc < dyn Any + Send + Sync > > , # [doc = " Extensions to the error."] # [serde (skip_serializing_if = "error_extensions_is_empty")] pub extensions : Option < ErrorExtensionValues > , }
};
}
