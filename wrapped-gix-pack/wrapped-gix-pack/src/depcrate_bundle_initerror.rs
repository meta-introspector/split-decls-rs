// Generated macro for Error (enum)
macro_rules! Depcrate_bundle_initError {
() => {
// Module: crate::bundle::init
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Returned by [`Bundle::at()`]"] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("An 'idx' extension is expected of an index file: '{0}'")] InvalidPath (PathBuf) , # [error (transparent)] Pack (# [from] crate :: data :: header :: decode :: Error) , # [error (transparent)] Index (# [from] crate :: index :: init :: Error) , }
};
}
