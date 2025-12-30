// Generated macro for read_to_string (function)
macro_rules! Depcrateread_to_string {
() => {
// Module: crate
// Provides: {"read_to_string"}
// Dependencies: {}
# [doc = " Reads the entire contents of a file as a string."] # [doc = ""] # [doc = " This is a convenience function for reading entire files. It pre-allocates a string based on the"] # [doc = " file size when available, so it is typically faster than manually opening a file and reading"] # [doc = " from it."] # [doc = ""] # [doc = " If you want to read the contents as raw bytes, use [`read()`] instead."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " An error will be returned in the following situations:"] # [doc = ""] # [doc = " * `path` does not point to an existing file."] # [doc = " * The current process lacks permissions to read the file."] # [doc = " * The contents of the file cannot be read as a UTF-8 string."] # [doc = " * Some other I/O error occurred."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " let contents = async_fs::read_to_string(\"a.txt\").await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub async fn read_to_string < P : AsRef < Path > > (path : P) -> io :: Result < String > { let path = path . as_ref () . to_owned () ; unblock (move | | std :: fs :: read_to_string (path)) . await }
};
}
