// Generated macro for impl_108 (impl)
macro_rules! Depcrateimpl_108 {
() => {
// Module: crate
// Provides: {"impl_108"}
// Dependencies: {}
impl TryFrom < PathBuf > for Utf8PathBuf { type Error = FromPathBufError ; fn try_from (path : PathBuf) -> Result < Utf8PathBuf , Self :: Error > { Utf8PathBuf :: from_path_buf (path) . map_err (| path | FromPathBufError { path , error : FromPathError (()) , }) } }
};
}
