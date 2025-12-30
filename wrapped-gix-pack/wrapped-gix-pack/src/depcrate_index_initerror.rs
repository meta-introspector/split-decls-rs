// Generated macro for Error (enum)
macro_rules! Depcrate_index_initError {
() => {
// Module: crate::index::init
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Returned by [`index::File::at()`]."] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("Could not open pack index file at '{path}'")] Io { source : std :: io :: Error , path : std :: path :: PathBuf , } , # [error ("{message}")] Corrupt { message : String } , # [error ("Unsupported index version: {version})")] UnsupportedVersion { version : u32 } , }
};
}
