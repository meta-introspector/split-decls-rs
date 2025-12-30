// Generated macro for canonicalize (function)
macro_rules! Depcratecanonicalize {
() => {
// Module: crate
// Provides: {"canonicalize"}
// Dependencies: {}
# [doc = " Returns the canonical form of a path."] # [doc = ""] # [doc = " The returned path is in absolute form with all intermediate components normalized and symbolic"] # [doc = " links resolved."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " An error will be returned in the following situations:"] # [doc = ""] # [doc = " * `path` does not point to an existing file or directory."] # [doc = " * A non-final component in `path` is not a directory."] # [doc = " * Some other I/O error occurred."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " let path = async_fs::canonicalize(\".\").await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub async fn canonicalize < P : AsRef < Path > > (path : P) -> io :: Result < PathBuf > { let path = path . as_ref () . to_owned () ; unblock (move | | std :: fs :: canonicalize (path)) . await }
};
}
