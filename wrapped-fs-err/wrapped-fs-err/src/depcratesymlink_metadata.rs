// Generated macro for symlink_metadata (function)
macro_rules! Depcratesymlink_metadata {
() => {
// Module: crate
// Provides: {"symlink_metadata"}
// Dependencies: {}
# [doc = " Query the metadata about a file without following symlinks."] # [doc = ""] # [doc = " Wrapper for [`fs::symlink_metadata`](https://doc.rust-lang.org/stable/std/fs/fn.symlink_metadata.html)."] pub fn symlink_metadata < P : AsRef < Path > > (path : P) -> io :: Result < fs :: Metadata > { let path = path . as_ref () ; fs :: symlink_metadata (path) . map_err (| source | Error :: build (source , ErrorKind :: SymlinkMetadata , path)) }
};
}
