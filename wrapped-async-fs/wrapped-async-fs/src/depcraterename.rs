// Generated macro for rename (function)
macro_rules! Depcraterename {
() => {
// Module: crate
// Provides: {"rename"}
// Dependencies: {}
# [doc = " Renames a file or directory to a new location."] # [doc = ""] # [doc = " If a file or directory already exists at the target location, it will be overwritten by this"] # [doc = " operation."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " An error will be returned in the following situations:"] # [doc = ""] # [doc = " * `src` does not point to an existing file or directory."] # [doc = " * `src` and `dst` are on different filesystems."] # [doc = " * The current process lacks permissions to do the rename operation."] # [doc = " * Some other I/O error occurred."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " async_fs::rename(\"a.txt\", \"b.txt\").await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub async fn rename < P : AsRef < Path > , Q : AsRef < Path > > (src : P , dst : Q) -> io :: Result < () > { let src = src . as_ref () . to_owned () ; let dst = dst . as_ref () . to_owned () ; unblock (move | | std :: fs :: rename (& src , & dst)) . await }
};
}
