// Generated macro for remove_file (function)
macro_rules! Depcrate_tokioremove_file {
() => {
// Module: crate::tokio
// Provides: {"remove_file"}
// Dependencies: {}
# [doc = " Removes a file from the filesystem."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::remove_file`]."] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub async fn remove_file (path : impl AsRef < Path >) -> io :: Result < () > { let path = path . as_ref () ; tokio :: fs :: remove_file (path) . await . map_err (| err | Error :: build (err , ErrorKind :: RemoveFile , path)) }
};
}
