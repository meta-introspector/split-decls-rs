// Generated macro for remove_dir_all (function)
macro_rules! Depcrate_tokioremove_dir_all {
() => {
// Module: crate::tokio
// Provides: {"remove_dir_all"}
// Dependencies: {}
# [doc = " Removes a directory at this path, after removing all its contents. Use carefully!"] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::remove_dir_all`]."] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub async fn remove_dir_all (path : impl AsRef < Path >) -> io :: Result < () > { let path = path . as_ref () ; tokio :: fs :: remove_dir_all (path) . await . map_err (| err | Error :: build (err , ErrorKind :: RemoveDir , path)) }
};
}
