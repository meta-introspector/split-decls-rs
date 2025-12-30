// Generated macro for metadata (function)
macro_rules! Depcrate_tokiometadata {
() => {
// Module: crate::tokio
// Provides: {"metadata"}
// Dependencies: {}
# [doc = " Given a path, queries the file system to get information about a file,"] # [doc = " directory, etc."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::metadata`]."] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub async fn metadata (path : impl AsRef < Path >) -> io :: Result < Metadata > { let path = path . as_ref () ; tokio :: fs :: metadata (path) . await . map_err (| err | Error :: build (err , ErrorKind :: Metadata , path)) }
};
}
