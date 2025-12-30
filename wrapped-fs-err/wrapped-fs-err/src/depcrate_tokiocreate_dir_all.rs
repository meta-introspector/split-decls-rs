// Generated macro for create_dir_all (function)
macro_rules! Depcrate_tokiocreate_dir_all {
() => {
// Module: crate::tokio
// Provides: {"create_dir_all"}
// Dependencies: {}
# [doc = " Recursively creates a directory and all of its parent components if they"] # [doc = " are missing."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::create_dir_all`]."] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub async fn create_dir_all (path : impl AsRef < Path >) -> io :: Result < () > { let path = path . as_ref () ; tokio :: fs :: create_dir_all (path) . await . map_err (| err | Error :: build (err , ErrorKind :: CreateDir , path)) }
};
}
