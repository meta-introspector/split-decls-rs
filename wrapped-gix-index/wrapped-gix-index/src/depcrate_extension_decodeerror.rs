// Generated macro for error (module)
macro_rules! Depcrate_extension_decodeerror {
() => {
// Module: crate::extension::decode
// Provides: {"error"}
// Dependencies: {}
mod error { use crate :: extension ; # [doc = " The error returned when decoding extensions."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Encountered mandatory extension '{}' which isn't implemented yet" , String :: from_utf8_lossy (signature))] MandatoryUnimplemented { signature : extension :: Signature } , # [error ("Could not parse mandatory link extension")] Link (# [from] extension :: link :: decode :: Error) , } }
};
}
