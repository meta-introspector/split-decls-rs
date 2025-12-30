// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl TryFrom < Utf8PathBuf > for AbsPathBuf { type Error = Utf8PathBuf ; fn try_from (path_buf : Utf8PathBuf) -> Result < AbsPathBuf , Utf8PathBuf > { if ! path_buf . is_absolute () { return Err (path_buf) ; } Ok (AbsPathBuf (path_buf)) } }
};
}
