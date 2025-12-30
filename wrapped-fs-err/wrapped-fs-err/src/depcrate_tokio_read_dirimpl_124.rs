// Generated macro for impl_124 (impl)
macro_rules! Depcrate_tokio_read_dirimpl_124 {
() => {
// Module: crate::tokio::read_dir
// Provides: {"impl_124"}
// Dependencies: {}
impl DirEntry { # [doc = " Returns the full path to the file that this entry represents."] # [doc = ""] # [doc = " Wrapper around [`tokio::fs::DirEntry::path`]."] pub fn path (& self) -> PathBuf { self . tokio . path () } # [doc = " Returns the bare file name of this directory entry without any other"] # [doc = " leading path component."] # [doc = ""] # [doc = " Wrapper around [`tokio::fs::DirEntry::file_name`]."] pub fn file_name (& self) -> OsString { self . tokio . file_name () } # [doc = " Returns the metadata for the file that this entry points at."] # [doc = ""] # [doc = " Wrapper around [`tokio::fs::DirEntry::metadata`]."] pub async fn metadata (& self) -> io :: Result < Metadata > { self . tokio . metadata () . await . map_err (| err | Error :: build (err , ErrorKind :: Metadata , self . path ())) } # [doc = " Returns the file type for the file that this entry points at."] # [doc = ""] # [doc = " Wrapper around [`tokio::fs::DirEntry::file_type`]."] pub async fn file_type (& self) -> io :: Result < FileType > { self . tokio . file_type () . await . map_err (| err | Error :: build (err , ErrorKind :: Metadata , self . path ())) } }
};
}
