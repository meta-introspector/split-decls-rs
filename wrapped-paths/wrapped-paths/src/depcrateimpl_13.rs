// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl TryFrom < & str > for AbsPathBuf { type Error = Utf8PathBuf ; fn try_from (path : & str) -> Result < AbsPathBuf , Utf8PathBuf > { AbsPathBuf :: try_from (Utf8PathBuf :: from (path)) } }
};
}
