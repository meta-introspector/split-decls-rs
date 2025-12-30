// Generated macro for hard_link (function)
macro_rules! Depcratehard_link {
() => {
// Module: crate
// Provides: {"hard_link"}
// Dependencies: {}
# [doc = " Creates a hard link on the filesystem."] # [doc = ""] # [doc = " The `dst` path will be a link pointing to the `src` path. Note that operating systems often"] # [doc = " require these two paths to be located on the same filesystem."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " An error will be returned in the following situations:"] # [doc = ""] # [doc = " * `src` does not point to an existing file."] # [doc = " * Some other I/O error occurred."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " async_fs::hard_link(\"a.txt\", \"b.txt\").await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub async fn hard_link < P : AsRef < Path > , Q : AsRef < Path > > (src : P , dst : Q) -> io :: Result < () > { let src = src . as_ref () . to_owned () ; let dst = dst . as_ref () . to_owned () ; unblock (move | | std :: fs :: hard_link (& src , & dst)) . await }
};
}
