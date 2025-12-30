// Generated macro for DirEntry (struct)
macro_rules! Depcrate_rootDirEntry {
() => {
// Module: crate::root
// Provides: {"DirEntry"}
// Dependencies: {}
# [doc = " Entries returned by the [`ReadDir`] iterator."] # [doc = ""] # [doc = " An instance of `DirEntry` represents an entry inside of a directory on the"] # [doc = " filesystem. Each entry can be inspected via methods to learn about the full"] # [doc = " path or possibly other metadata through per-platform extension traits."] # [doc = ""] # [doc = " # Platform-specific behavior"] # [doc = ""] # [doc = " On Unix, the `DirEntry` struct contains an internal reference to the open"] # [doc = " directory. Holding `DirEntry` objects will consume a file handle even after"] # [doc = " the `ReadDir` iterator is dropped."] pub struct DirEntry { inner : imp :: DirEntry , }
};
}
