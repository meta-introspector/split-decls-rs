// Generated macro for copy (function)
macro_rules! Depcratecopy {
() => {
// Module: crate
// Provides: {"copy"}
// Dependencies: {}
# [doc = " Copies a file to a new location."] # [doc = ""] # [doc = " On success, the total number of bytes copied is returned and equals the length of the `dst`"] # [doc = " file after this operation."] # [doc = ""] # [doc = " The old contents of `dst` will be overwritten. If `src` and `dst` both point to the same"] # [doc = " file, then the file will likely get truncated as a result of this operation."] # [doc = ""] # [doc = " If you're working with open [`File`]s and want to copy contents through those types, use"] # [doc = " [`futures_lite::io::copy()`] instead."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " An error will be returned in the following situations:"] # [doc = ""] # [doc = " * `src` does not point to an existing file."] # [doc = " * The current process lacks permissions to read `src` or write `dst`."] # [doc = " * Some other I/O error occurred."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " let num_bytes = async_fs::copy(\"a.txt\", \"b.txt\").await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub async fn copy < P : AsRef < Path > , Q : AsRef < Path > > (src : P , dst : Q) -> io :: Result < u64 > { let src = src . as_ref () . to_owned () ; let dst = dst . as_ref () . to_owned () ; unblock (move | | std :: fs :: copy (& src , & dst)) . await }
};
}
