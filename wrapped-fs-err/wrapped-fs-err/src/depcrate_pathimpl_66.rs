// Generated macro for impl_66 (impl)
macro_rules! Depcrate_pathimpl_66 {
() => {
// Module: crate::path
// Provides: {"impl_66"}
// Dependencies: {}
impl PathExt for Path { # [cfg (rustc_1_63)] fn fs_err_try_exists (& self) -> io :: Result < bool > { self . try_exists () . map_err (| source | Error :: build (source , ErrorKind :: FileExists , self)) } fn fs_err_metadata (& self) -> io :: Result < fs :: Metadata > { crate :: metadata (self) } fn fs_err_symlink_metadata (& self) -> io :: Result < fs :: Metadata > { crate :: symlink_metadata (self) } fn fs_err_canonicalize (& self) -> io :: Result < PathBuf > { crate :: canonicalize (self) } fn fs_err_read_link (& self) -> io :: Result < PathBuf > { crate :: read_link (self) } fn fs_err_read_dir (& self) -> io :: Result < crate :: ReadDir > { crate :: read_dir (self) } }
};
}
