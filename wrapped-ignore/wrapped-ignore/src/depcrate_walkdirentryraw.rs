// Generated macro for DirEntryRaw (struct)
macro_rules! Depcrate_walkDirEntryRaw {
() => {
// Module: crate::walk
// Provides: {"DirEntryRaw"}
// Dependencies: {}
# [doc = " DirEntryRaw is essentially copied from the walkdir crate so that we can"] # [doc = " build `DirEntry`s from whole cloth in the parallel iterator."] # [derive (Clone)] struct DirEntryRaw { # [doc = " The path as reported by the `fs::ReadDir` iterator (even if it's a"] # [doc = " symbolic link)."] path : PathBuf , # [doc = " The file type. Necessary for recursive iteration, so store it."] ty : FileType , # [doc = " Is set when this entry was created from a symbolic link and the user"] # [doc = " expects the iterator to follow symbolic links."] follow_link : bool , # [doc = " The depth at which this entry was generated relative to the root."] depth : usize , # [doc = " The underlying inode number (Unix only)."] # [cfg (unix)] ino : u64 , # [doc = " The underlying metadata (Windows only). We store this on Windows"] # [doc = " because this comes for free while reading a directory."] # [cfg (windows)] metadata : fs :: Metadata , }
};
}
