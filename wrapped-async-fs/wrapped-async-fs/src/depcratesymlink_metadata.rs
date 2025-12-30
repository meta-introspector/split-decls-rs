// Generated macro for symlink_metadata (function)
macro_rules! Depcratesymlink_metadata {
() => {
// Module: crate
// Provides: {"symlink_metadata"}
// Dependencies: {}
# [doc = " Reads metadata for a path without following symbolic links."] # [doc = ""] # [doc = " If you want to follow symbolic links before reading metadata of the target file or directory,"] # [doc = " use [`metadata()`] instead."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " An error will be returned in the following situations:"] # [doc = ""] # [doc = " * `path` does not point to an existing file or directory."] # [doc = " * The current process lacks permissions to read metadata for the path."] # [doc = " * Some other I/O error occurred."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " let perm = async_fs::symlink_metadata(\"a.txt\").await?.permissions();"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub async fn symlink_metadata < P : AsRef < Path > > (path : P) -> io :: Result < Metadata > { let path = path . as_ref () . to_owned () ; unblock (move | | std :: fs :: symlink_metadata (path)) . await }
};
}
