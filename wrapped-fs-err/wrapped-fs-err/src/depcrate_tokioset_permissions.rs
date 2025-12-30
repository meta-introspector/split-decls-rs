// Generated macro for set_permissions (function)
macro_rules! Depcrate_tokioset_permissions {
() => {
// Module: crate::tokio
// Provides: {"set_permissions"}
// Dependencies: {}
# [doc = " Changes the permissions found on a file or a directory."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::set_permissions`]."] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub async fn set_permissions (path : impl AsRef < Path > , perm : Permissions) -> io :: Result < () > { let path = path . as_ref () ; tokio :: fs :: set_permissions (path , perm) . await . map_err (| err | Error :: build (err , ErrorKind :: SetPermissions , path)) }
};
}
