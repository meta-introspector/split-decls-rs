// Generated macro for DirEntry (struct)
macro_rules! DepcrateDirEntry {
() => {
// Module: crate
// Provides: {"DirEntry"}
// Dependencies: {}
# [doc = " An entry in a directory."] # [doc = ""] # [doc = " A stream of entries in a directory is returned by [`read_dir()`]."] # [doc = ""] # [doc = " For Unix-specific options, import the [`DirEntryExt`][`std::os::unix::fs::DirEntryExt`] trait."] pub struct DirEntry (Arc < std :: fs :: DirEntry >) ;
};
}
