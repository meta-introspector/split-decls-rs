// Generated macro for remove_file (function)
macro_rules! Depcrateremove_file {
() => {
// Module: crate
// Provides: {"remove_file"}
// Dependencies: {}
# [doc = " Removes a file."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " An error will be returned in the following situations:"] # [doc = ""] # [doc = " * `path` does not point to an existing file."] # [doc = " * The current process lacks permissions to remove the file."] # [doc = " * Some other I/O error occurred."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " async_fs::remove_file(\"a.txt\").await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub async fn remove_file < P : AsRef < Path > > (path : P) -> io :: Result < () > { let path = path . as_ref () . to_owned () ; unblock (move | | std :: fs :: remove_file (path)) . await }
};
}
