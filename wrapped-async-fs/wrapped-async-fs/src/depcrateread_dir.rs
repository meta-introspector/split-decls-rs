// Generated macro for read_dir (function)
macro_rules! Depcrateread_dir {
() => {
// Module: crate
// Provides: {"read_dir"}
// Dependencies: {}
# [doc = " Returns a stream of entries in a directory."] # [doc = ""] # [doc = " The stream yields items of type [`io::Result`]`<`[`DirEntry`]`>`. Note that I/O errors can"] # [doc = " occur while reading from the stream."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " An error will be returned in the following situations:"] # [doc = ""] # [doc = " * `path` does not point to an existing directory."] # [doc = " * The current process lacks permissions to read the contents of the directory."] # [doc = " * Some other I/O error occurred."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " use futures_lite::stream::StreamExt;"] # [doc = ""] # [doc = " let mut entries = async_fs::read_dir(\".\").await?;"] # [doc = ""] # [doc = " while let Some(entry) = entries.try_next().await? {"] # [doc = "     println!(\"{}\", entry.file_name().to_string_lossy());"] # [doc = " }"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub async fn read_dir < P : AsRef < Path > > (path : P) -> io :: Result < ReadDir > { let path = path . as_ref () . to_owned () ; unblock (move | | std :: fs :: read_dir (path) . map (| inner | ReadDir (State :: Idle (Some (inner))))) . await }
};
}
