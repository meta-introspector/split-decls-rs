// Generated macro for metadata (function)
macro_rules! Depcratemetadata {
() => {
// Module: crate
// Provides: {"metadata"}
// Dependencies: {}
# [doc = " Reads metadata for a path."] # [doc = ""] # [doc = " This function will traverse symbolic links to read metadata for the target file or directory."] # [doc = " If you want to read metadata without following symbolic links, use [`symlink_metadata()`]"] # [doc = " instead."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " An error will be returned in the following situations:"] # [doc = ""] # [doc = " * `path` does not point to an existing file or directory."] # [doc = " * The current process lacks permissions to read metadata for the path."] # [doc = " * Some other I/O error occurred."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " let perm = async_fs::metadata(\"a.txt\").await?.permissions();"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub async fn metadata < P : AsRef < Path > > (path : P) -> io :: Result < Metadata > { let path = path . as_ref () . to_owned () ; unblock (move | | std :: fs :: metadata (path)) . await }
};
}
