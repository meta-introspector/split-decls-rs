// Generated macro for set_permissions (function)
macro_rules! Depcrateset_permissions {
() => {
// Module: crate
// Provides: {"set_permissions"}
// Dependencies: {}
# [doc = " Changes the permissions of a file or directory."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " An error will be returned in the following situations:"] # [doc = ""] # [doc = " * `path` does not point to an existing file or directory."] # [doc = " * The current process lacks permissions to change attributes on the file or directory."] # [doc = " * Some other I/O error occurred."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " let mut perm = async_fs::metadata(\"a.txt\").await?.permissions();"] # [doc = " perm.set_readonly(true);"] # [doc = " async_fs::set_permissions(\"a.txt\", perm).await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub async fn set_permissions < P : AsRef < Path > > (path : P , perm : Permissions) -> io :: Result < () > { let path = path . as_ref () . to_owned () ; unblock (move | | std :: fs :: set_permissions (path , perm)) . await }
};
}
