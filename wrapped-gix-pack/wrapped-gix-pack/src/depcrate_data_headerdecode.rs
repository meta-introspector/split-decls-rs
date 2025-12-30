// Generated macro for decode (module)
macro_rules! Depcrate_data_headerdecode {
() => {
// Module: crate::data::header
// Provides: {"decode"}
// Dependencies: {}
# [doc = ""] pub mod decode { # [doc = " Returned by [`decode()`][super::decode()]."] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("Could not open pack file at '{path}'")] Io { source : std :: io :: Error , path : std :: path :: PathBuf , } , # [error ("{0}")] Corrupt (String) , # [error ("Unsupported pack version: {0}")] UnsupportedVersion (u32) , } }
};
}
