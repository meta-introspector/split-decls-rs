// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl TryFrom < Utf8PathBuf > for RelPathBuf { type Error = Utf8PathBuf ; fn try_from (path_buf : Utf8PathBuf) -> Result < RelPathBuf , Utf8PathBuf > { if ! path_buf . is_relative () { return Err (path_buf) ; } Ok (RelPathBuf (path_buf)) } }
};
}
