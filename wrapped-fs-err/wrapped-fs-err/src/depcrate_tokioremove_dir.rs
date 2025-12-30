// Generated macro for remove_dir (function)
macro_rules! Depcrate_tokioremove_dir {
() => {
// Module: crate::tokio
// Provides: {"remove_dir"}
// Dependencies: {}
# [doc = " Removes an existing, empty directory."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::remove_dir`]."] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub async fn remove_dir (path : impl AsRef < Path >) -> io :: Result < () > { let path = path . as_ref () ; tokio :: fs :: remove_dir (path) . await . map_err (| err | Error :: build (err , ErrorKind :: RemoveDir , path)) }
};
}
