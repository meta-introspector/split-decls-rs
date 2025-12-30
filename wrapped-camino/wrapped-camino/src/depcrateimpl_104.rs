// Generated macro for impl_104 (impl)
macro_rules! Depcrateimpl_104 {
() => {
// Module: crate
// Provides: {"impl_104"}
// Dependencies: {}
impl From < Utf8PathBuf > for Box < Path > { fn from (path : Utf8PathBuf) -> Box < Path > { PathBuf :: from (path) . into_boxed_path () } }
};
}
