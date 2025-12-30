// Generated macro for symlink_metadata (function)
macro_rules! Depcrate_tokiosymlink_metadata {
() => {
// Module: crate::tokio
// Provides: {"symlink_metadata"}
// Dependencies: {}
# [doc = " Queries the file system metadata for a path."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::symlink_metadata`]."] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub async fn symlink_metadata (path : impl AsRef < Path >) -> io :: Result < Metadata > { let path = path . as_ref () ; tokio :: fs :: symlink_metadata (path) . await . map_err (| err | Error :: build (err , ErrorKind :: SymlinkMetadata , path)) }
};
}
