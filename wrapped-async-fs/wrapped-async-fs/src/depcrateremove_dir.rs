// Generated macro for remove_dir (function)
macro_rules! Depcrateremove_dir {
() => {
// Module: crate
// Provides: {"remove_dir"}
// Dependencies: {}
# [doc = " Removes an empty directory."] # [doc = ""] # [doc = " Note that this function can only delete an empty directory. If you want to delete a directory"] # [doc = " and all of its contents, use [`remove_dir_all()`] instead."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " An error will be returned in the following situations:"] # [doc = ""] # [doc = " * `path` is not an existing and empty directory."] # [doc = " * The current process lacks permissions to remove the directory."] # [doc = " * Some other I/O error occurred."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " async_fs::remove_dir(\"./some/directory\").await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub async fn remove_dir < P : AsRef < Path > > (path : P) -> io :: Result < () > { let path = path . as_ref () . to_owned () ; unblock (move | | std :: fs :: remove_dir (path)) . await }
};
}
