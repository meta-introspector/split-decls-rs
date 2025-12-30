// Generated macro for PathExt (trait)
macro_rules! Depcrate_pathPathExt {
() => {
// Module: crate::path
// Provides: {"PathExt"}
// Dependencies: {}
# [doc = " Defines aliases on [`Path`](https://doc.rust-lang.org/std/path/struct.Path.html) for `fs_err` functions."] # [doc = ""] # [doc = " This trait is sealed and can not be implemented by other crates."] pub trait PathExt : crate :: Sealed { # [doc = " Returns Ok(true) if the path points at an existing entity."] # [doc = ""] # [doc = " Wrapper for [`Path::try_exists`](https://doc.rust-lang.org/std/path/struct.Path.html#method.try_exists)."] # [cfg (rustc_1_63)] fn fs_err_try_exists (& self) -> io :: Result < bool > ; # [doc = " Given a path, query the file system to get information about a file, directory, etc."] # [doc = ""] # [doc = " Wrapper for [`crate::metadata`]."] fn fs_err_metadata (& self) -> io :: Result < fs :: Metadata > ; # [doc = " Query the metadata about a file without following symlinks."] # [doc = ""] # [doc = " Wrapper for [`crate::symlink_metadata`]."] fn fs_err_symlink_metadata (& self) -> io :: Result < fs :: Metadata > ; # [doc = " Returns the canonical, absolute form of a path with all intermediate components"] # [doc = " normalized and symbolic links resolved."] # [doc = ""] # [doc = " Wrapper for [`crate::canonicalize`]."] fn fs_err_canonicalize (& self) -> io :: Result < PathBuf > ; # [doc = " Reads a symbolic link, returning the file that the link points to."] # [doc = ""] # [doc = " Wrapper for [`crate::read_link`]."] fn fs_err_read_link (& self) -> io :: Result < PathBuf > ; # [doc = " Returns an iterator over the entries within a directory."] # [doc = ""] # [doc = " Wrapper for [`crate::read_dir`]."] fn fs_err_read_dir (& self) -> io :: Result < crate :: ReadDir > ; }
};
}
