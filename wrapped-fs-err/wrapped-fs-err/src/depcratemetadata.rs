// Generated macro for metadata (function)
macro_rules! Depcratemetadata {
() => {
// Module: crate
// Provides: {"metadata"}
// Dependencies: {}
# [doc = " Given a path, query the file system to get information about a file, directory, etc."] # [doc = ""] # [doc = " Wrapper for [`fs::metadata`](https://doc.rust-lang.org/stable/std/fs/fn.metadata.html)."] pub fn metadata < P : AsRef < Path > > (path : P) -> io :: Result < fs :: Metadata > { let path = path . as_ref () ; fs :: metadata (path) . map_err (| source | Error :: build (source , ErrorKind :: Metadata , path)) }
};
}
