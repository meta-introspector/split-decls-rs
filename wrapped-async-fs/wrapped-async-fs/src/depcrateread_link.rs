// Generated macro for read_link (function)
macro_rules! Depcrateread_link {
() => {
// Module: crate
// Provides: {"read_link"}
// Dependencies: {}
# [doc = " Reads a symbolic link and returns the path it points to."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " An error will be returned in the following situations:"] # [doc = ""] # [doc = " * `path` does not point to an existing link."] # [doc = " * Some other I/O error occurred."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " let path = async_fs::read_link(\"a.txt\").await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub async fn read_link < P : AsRef < Path > > (path : P) -> io :: Result < PathBuf > { let path = path . as_ref () . to_owned () ; unblock (move | | std :: fs :: read_link (path)) . await }
};
}
