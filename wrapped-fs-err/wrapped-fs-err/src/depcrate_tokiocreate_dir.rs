// Generated macro for create_dir (function)
macro_rules! Depcrate_tokiocreate_dir {
() => {
// Module: crate::tokio
// Provides: {"create_dir"}
// Dependencies: {}
# [doc = " Creates a new, empty directory at the provided path."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::create_dir`]."] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub async fn create_dir (path : impl AsRef < Path >) -> io :: Result < () > { let path = path . as_ref () ; tokio :: fs :: create_dir (path) . await . map_err (| err | Error :: build (err , ErrorKind :: CreateDir , path)) }
};
}
