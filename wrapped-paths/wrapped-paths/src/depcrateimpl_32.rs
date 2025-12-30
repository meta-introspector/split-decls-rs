// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl TryFrom < & str > for RelPathBuf { type Error = Utf8PathBuf ; fn try_from (path : & str) -> Result < RelPathBuf , Utf8PathBuf > { RelPathBuf :: try_from (Utf8PathBuf :: from (path)) } }
};
}
