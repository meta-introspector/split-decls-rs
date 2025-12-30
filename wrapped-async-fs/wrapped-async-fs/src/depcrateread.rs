// Generated macro for read (function)
macro_rules! Depcrateread {
() => {
// Module: crate
// Provides: {"read"}
// Dependencies: {}
# [doc = " Reads the entire contents of a file as raw bytes."] # [doc = ""] # [doc = " This is a convenience function for reading entire files. It pre-allocates a buffer based on the"] # [doc = " file size when available, so it is typically faster than manually opening a file and reading"] # [doc = " from it."] # [doc = ""] # [doc = " If you want to read the contents as a string, use [`read_to_string()`] instead."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " An error will be returned in the following situations:"] # [doc = ""] # [doc = " * `path` does not point to an existing file."] # [doc = " * The current process lacks permissions to read the file."] # [doc = " * Some other I/O error occurred."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " let contents = async_fs::read(\"a.txt\").await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub async fn read < P : AsRef < Path > > (path : P) -> io :: Result < Vec < u8 > > { let path = path . as_ref () . to_owned () ; unblock (move | | std :: fs :: read (path)) . await }
};
}
