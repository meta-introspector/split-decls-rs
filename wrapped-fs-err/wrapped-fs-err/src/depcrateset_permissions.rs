// Generated macro for set_permissions (function)
macro_rules! Depcrateset_permissions {
() => {
// Module: crate
// Provides: {"set_permissions"}
// Dependencies: {}
# [doc = " Changes the permissions found on a file or a directory."] # [doc = ""] # [doc = " Wrapper for [`fs::set_permissions`](https://doc.rust-lang.org/stable/std/fs/fn.set_permissions.html)."] pub fn set_permissions < P : AsRef < Path > > (path : P , perm : fs :: Permissions) -> io :: Result < () > { let path = path . as_ref () ; fs :: set_permissions (path , perm) . map_err (| source | Error :: build (source , ErrorKind :: SetPermissions , path)) }
};
}
