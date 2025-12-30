// Generated macro for impl_11 (impl)
macro_rules! Depcrate_dirimpl_11 {
() => {
// Module: crate::dir
// Provides: {"impl_11"}
// Dependencies: {}
impl DirEntry { # [doc = " Returns the full path to the file that this entry represents."] # [doc = ""] # [doc = " Wrapper for [`DirEntry::path`](https://doc.rust-lang.org/stable/std/fs/struct.DirEntry.html#method.path)."] pub fn path (& self) -> PathBuf { self . inner . path () } # [doc = " Returns the metadata for the file that this entry points at."] # [doc = ""] # [doc = " Wrapper for [`DirEntry::metadata`](https://doc.rust-lang.org/stable/std/fs/struct.DirEntry.html#method.metadata)."] pub fn metadata (& self) -> io :: Result < fs :: Metadata > { self . inner . metadata () . map_err (| source | Error :: build (source , ErrorKind :: Metadata , self . path ())) } # [doc = " Returns the file type for the file that this entry points at."] # [doc = ""] # [doc = " Wrapper for [`DirEntry::file_type`](https://doc.rust-lang.org/stable/std/fs/struct.DirEntry.html#method.file_type)."] pub fn file_type (& self) -> io :: Result < fs :: FileType > { self . inner . file_type () . map_err (| source | Error :: build (source , ErrorKind :: Metadata , self . path ())) } # [doc = " Returns the file name of this directory entry without any leading path component(s)."] # [doc = ""] # [doc = " Wrapper for [`DirEntry::file_name`](https://doc.rust-lang.org/stable/std/fs/struct.DirEntry.html#method.file_name)."] pub fn file_name (& self) -> OsString { self . inner . file_name () } }
};
}
