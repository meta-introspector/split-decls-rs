// Generated macro for Error (enum)
macro_rules! Depcrate_blob_platform_mergeError {
() => {
// Module: crate::blob::platform::merge
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`PlatformRef::merge()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] PrepareExternalDriver (# [from] inner :: prepare_external_driver :: Error) , # [error ("Failed to launch external merge driver: {cmd}")] SpawnExternalDriver { cmd : String , source : std :: io :: Error } , # [error ("External merge driver failed with non-zero exit status {status:?}: {cmd}")] ExternalDriverFailure { status : std :: process :: ExitStatus , cmd : String , } , # [error ("IO failed when dealing with merge-driver output")] ExternalDriverIO (# [from] std :: io :: Error) , }
};
}
