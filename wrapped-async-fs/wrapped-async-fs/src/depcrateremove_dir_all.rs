// Generated macro for remove_dir_all (function)
macro_rules! Depcrateremove_dir_all {
() => {
// Module: crate
// Provides: {"remove_dir_all"}
// Dependencies: {}
# [doc = " Removes a directory and all of its contents."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " An error will be returned in the following situations:"] # [doc = ""] # [doc = " * `path` is not an existing directory."] # [doc = " * The current process lacks permissions to remove the directory."] # [doc = " * Some other I/O error occurred."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " async_fs::remove_dir_all(\"./some/directory\").await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub async fn remove_dir_all < P : AsRef < Path > > (path : P) -> io :: Result < () > { let path = path . as_ref () . to_owned () ; unblock (move | | std :: fs :: remove_dir_all (path)) . await }
};
}
