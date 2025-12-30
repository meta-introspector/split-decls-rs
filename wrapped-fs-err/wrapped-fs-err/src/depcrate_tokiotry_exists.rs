// Generated macro for try_exists (function)
macro_rules! Depcrate_tokiotry_exists {
() => {
// Module: crate::tokio
// Provides: {"try_exists"}
// Dependencies: {}
# [doc = " Returns `Ok(true)` if the path points at an existing entity."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::try_exists`]."] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub async fn try_exists (path : impl AsRef < Path >) -> io :: Result < bool > { let path = path . as_ref () ; tokio :: fs :: try_exists (path) . await . map_err (| err | Error :: build (err , ErrorKind :: FileExists , path)) }
};
}
